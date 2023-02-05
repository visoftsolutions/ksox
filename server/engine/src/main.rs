#![allow(dead_code)] //TODO remove !!
#![allow(unused_variables)] //TODO remove !!
#![feature(let_chains)]

pub mod base {
    tonic::include_proto!("server.engine.base");
}

mod deserializer;
mod matching_engine;
mod serializer;
mod shutdown_signal;

use std::net::SocketAddr;

use anyhow::{Ok, Result};
use base::{
    engine_server::{Engine, EngineServer},
    CancelRequest, CancelResponse, DrainOrderBookRequest, DrainOrderBookResponse,
    DrainTradesRequest, DrainTradesResponse, SubmitRequest, SubmitResponse,
};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status};

#[derive(Debug, Default)]
pub struct EngineService;

#[tonic::async_trait]
impl Engine for EngineService {
    type DrainOrderBookStream = ReceiverStream<Result<DrainOrderBookResponse, Status>>;
    type DrainTradesStream = ReceiverStream<Result<DrainTradesResponse, Status>>;

    async fn submit(
        &self,
        _request: Request<SubmitRequest>,
    ) -> Result<Response<SubmitResponse>, Status> {
        unimplemented!()
    }

    async fn cancel(
        &self,
        _request: Request<CancelRequest>,
    ) -> Result<Response<CancelResponse>, Status> {
        unimplemented!()
    }

    async fn drain_order_book(
        &self,
        _request: Request<DrainOrderBookRequest>,
    ) -> Result<Response<Self::DrainOrderBookStream>, Status> {
        unimplemented!()
    }

    async fn drain_trades(
        &self,
        _request: Request<DrainTradesRequest>,
    ) -> Result<Response<Self::DrainTradesStream>, Status> {
        unimplemented!()
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr = SocketAddr::from(([0, 0, 0, 0], 80));

    let engine = EngineService::default();
    let svc = EngineServer::new(engine);

    tracing::info!("listening on {}", addr);

    Server::builder()
        .add_service(svc)
        .serve_with_shutdown(addr, shutdown_signal::listen())
        .await?;

    Ok(())
}
