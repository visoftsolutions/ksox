#![feature(let_chains)]

pub mod base {
    tonic::include_proto!("server.engine.base");
}

mod database;
mod health;
mod matching_engine;
mod shutdown_signal;

use std::net::SocketAddr;

use base::engine_server::EngineServer;
use sqlx::PgPool;
use tonic::transport::Server;

use crate::{
    base::health_check_response::ServingStatus, health::health_reporter,
    matching_engine::MatchingEngine,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let (mut health_reporter, health_server) = health_reporter();

    let database = PgPool::connect(std::env::var("DATABASE_URL")?.as_str()).await?;

    let matching_engine = MatchingEngine::new(
        database,
        std::env::var("ENGINE_FRACTION_ACCURACY")?.parse()?,
    );

    let addr = SocketAddr::from(([0, 0, 0, 0], 80));
    tracing::info!("listening on {}", addr);

    health_reporter
        .set_service_status("engine", ServingStatus::Serving)
        .await;

    Server::builder()
        .add_service(EngineServer::new(matching_engine))
        .add_service(health_server)
        .serve_with_shutdown(addr, shutdown_signal::listen())
        .await?;

    Ok(())
}
