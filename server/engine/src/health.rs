use std::{collections::HashMap, pin::Pin, sync::Arc};

use futures::stream::Stream;
use tokio::sync::{watch, RwLock};
#[cfg(feature = "transport")]
use tonic::server::NamedService;
use tonic::{Request, Response, Status};

use crate::base::{
    health_check_response::ServingStatus,
    health_server::{Health, HealthServer},
    HealthCheckRequest, HealthCheckResponse,
};

/// Creates a `HealthReporter` and a linked `HealthServer` pair. Together,
/// these types can be used to serve the gRPC Health Checking service.
///
/// A `HealthReporter` is used to update the state of gRPC services.
///
/// A `HealthServer` is a Tonic gRPC server for the `grpc.health.v1.Health`,
/// which can be added to a Tonic runtime using `add_service` on the runtime
/// builder.
pub fn health_reporter() -> (HealthReporter, HealthServer<impl Health>) {
    let reporter = HealthReporter::new();
    let service = HealthService::new(reporter.statuses.clone());
    let server = HealthServer::new(service);

    (reporter, server)
}

type StatusPair = (watch::Sender<ServingStatus>, watch::Receiver<ServingStatus>);

/// A handle providing methods to update the health status of gRPC services. A
/// `HealthReporter` is connected to a `HealthServer` which serves the statuses
/// over the `grpc.health.v1.Health` service.
#[derive(Clone, Debug)]
pub struct HealthReporter {
    statuses: Arc<RwLock<HashMap<String, StatusPair>>>,
}

impl HealthReporter {
    fn new() -> Self {
        // According to the gRPC Health Check specification, the empty service "" corresponds to the overall server health
        let server_status = ("".to_string(), watch::channel(ServingStatus::Serving));

        let statuses = Arc::new(RwLock::new(HashMap::from([server_status])));

        HealthReporter { statuses }
    }

    /// Sets the status of the service implemented by `S` to `Serving`. This notifies any watchers
    /// if there is a change in status.
    #[cfg(feature = "transport")]
    #[cfg_attr(docsrs, doc(cfg(feature = "transport")))]
    pub async fn set_serving<S>(&mut self)
    where
        S: NamedService,
    {
        let service_name = <S as NamedService>::NAME;
        self.set_service_status(service_name, ServingStatus::Serving)
            .await;
    }

    /// Sets the status of the service implemented by `S` to `NotServing`. This notifies any watchers
    /// if there is a change in status.
    #[cfg(feature = "transport")]
    #[cfg_attr(docsrs, doc(cfg(feature = "transport")))]
    pub async fn set_not_serving<S>(&mut self)
    where
        S: NamedService,
    {
        let service_name = <S as NamedService>::NAME;
        self.set_service_status(service_name, ServingStatus::NotServing)
            .await;
    }

    /// Sets the status of the service with `service_name` to `status`. This notifies any watchers
    /// if there is a change in status.
    pub async fn set_service_status<S>(&mut self, service_name: S, status: ServingStatus)
    where
        S: AsRef<str>,
    {
        let service_name = service_name.as_ref();
        let mut writer = self.statuses.write().await;
        match writer.get(service_name) {
            Some((tx, _)) => {
                // We only ever hand out clones of the receiver, so the originally-created
                // receiver should always be present, only being dropped when clearing the
                // service status. Consequently, `tx.send` should not fail, making use
                // of `expect` here safe.
                tx.send(status).expect("channel should not be closed");
            }
            None => {
                writer.insert(service_name.to_string(), watch::channel(status));
            }
        };
    }

    // Clear the status of the given service.
    // pub async fn clear_service_status(&mut self, service_name: &str) {
    //     let mut writer = self.statuses.write().await;
    //     let _ = writer.remove(service_name);
    // }
}

/// A service providing implementations of gRPC health checking protocol.
#[derive(Debug)]
pub struct HealthService {
    statuses: Arc<RwLock<HashMap<String, StatusPair>>>,
}

impl HealthService {
    fn new(services: Arc<RwLock<HashMap<String, StatusPair>>>) -> Self {
        HealthService { statuses: services }
    }

    async fn service_health(&self, service_name: &str) -> Option<ServingStatus> {
        let reader = self.statuses.read().await;
        reader.get(service_name).map(|p| *p.1.borrow())
    }
}

#[tonic::async_trait]
impl Health for HealthService {
    async fn check(
        &self,
        request: Request<HealthCheckRequest>,
    ) -> Result<Response<HealthCheckResponse>, Status> {
        let service_name = request.get_ref().service.as_str();
        let status = self.service_health(service_name).await;

        match status {
            None => Err(Status::not_found("service not registered")),
            Some(status) => Ok(Response::new(HealthCheckResponse {
                status: status as i32,
            })),
        }
    }

    type WatchStream =
        Pin<Box<dyn Stream<Item = Result<HealthCheckResponse, Status>> + Send + 'static>>;

    async fn watch(
        &self,
        request: Request<HealthCheckRequest>,
    ) -> Result<Response<Self::WatchStream>, Status> {
        let service_name = request.get_ref().service.as_str();
        let mut status_rx = match self.statuses.read().await.get(service_name) {
            None => return Err(Status::not_found("service not registered")),
            Some(pair) => pair.1.clone(),
        };

        let output = async_stream::try_stream! {
            // yield the current value
            let status = (*status_rx.borrow()) as i32;
            yield HealthCheckResponse { status };

            #[allow(clippy::redundant_pattern_matching)]
            while let Ok(_) = status_rx.changed().await {
                let status = (*status_rx.borrow()) as i32;
                yield HealthCheckResponse { status };
            }
        };

        Ok(Response::new(Box::pin(output) as Self::WatchStream))
    }
}
