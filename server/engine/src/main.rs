#![feature(let_chains)]

pub mod base {
    tonic::include_proto!("server.engine.base");
}

mod matching_engine;
mod shutdown_signal;

use std::{net::SocketAddr, str::FromStr};

use base::{
    engine_server::{Engine, EngineServer},
    CancelRequest, CancelResponse, SubmitRequest, SubmitResponse,
};
use database::{
    sqlx::{PgPool, Pool, Postgres},
    types::Volume,
};
use matching_engine::{models::MatchingEngineRequest, MatchingEngine};
use tonic::{transport::Server, Request, Response, Status};
use uuid::Uuid;

pub struct EngineService {
    engine: MatchingEngine,
}

impl EngineService {
    pub fn new(database: Pool<Postgres>) -> Self {
        EngineService {
            engine: MatchingEngine::new(database),
        }
    }
}

impl TryFrom<SubmitRequest> for MatchingEngineRequest {
    type Error = Status;
    fn try_from(value: SubmitRequest) -> std::result::Result<Self, Self::Error> {
        Ok(MatchingEngineRequest {
            user_id: Uuid::from_str(&value.user_id)
                .map_err(|err| Status::invalid_argument(err.to_string()))?,
            quote_asset_id: Uuid::from_str(&value.quote_asset_id)
                .map_err(|err| Status::invalid_argument(err.to_string()))?,
            base_asset_id: Uuid::from_str(&value.base_asset_id)
                .map_err(|err| Status::invalid_argument(err.to_string()))?,
            quote_asset_volume: Volume::from_str(&value.quote_asset_volume)
                .map_err(|err| Status::invalid_argument(err.to_string()))?,
            base_asset_volume: Volume::from_str(&value.base_asset_volume)
                .map_err(|err| Status::invalid_argument(err.to_string()))?,
        })
    }
}

#[tonic::async_trait]
impl Engine for EngineService {
    async fn submit(
        &self,
        request: Request<SubmitRequest>,
    ) -> Result<Response<SubmitResponse>, Status> {
        let request: MatchingEngineRequest = request.into_inner().try_into()?;
        self.engine
            .execute_request(request)
            .await
            .map_err(|err| Status::internal(err.to_string()))?;
        Ok(Response::new(SubmitResponse {}))
    }

    async fn cancel(
        &self,
        request: Request<CancelRequest>,
    ) -> Result<Response<CancelResponse>, Status> {
        let order_id = Uuid::from_str(&request.into_inner().order_id)
            .map_err(|err| Status::invalid_argument(err.to_string()))?;
        self.engine
            .cancel_order(order_id)
            .await
            .map_err(|err| Status::internal(err.to_string()))?;
        Ok(Response::new(CancelResponse {}))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let database = PgPool::connect(std::env::var("DATABASE_URL").unwrap_or_default().as_str())
        .await
        .unwrap();

    let engine = EngineService::new(database);

    let svc = EngineServer::new(engine);
    let addr = SocketAddr::from(([0, 0, 0, 0], 80));
    tracing::info!("listening on {}", addr);

    Server::builder()
        .add_service(svc)
        .serve_with_shutdown(addr, shutdown_signal::listen())
        .await?;

    Ok(())
}
