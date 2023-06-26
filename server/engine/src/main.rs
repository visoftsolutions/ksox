#![feature(let_chains)]

pub mod base {
    tonic::include_proto!("server.engine.base");
}

mod database;
mod matching_engine;
mod shutdown_signal;

use std::net::SocketAddr;

use base::engine_server::EngineServer;
use sqlx::PgPool;
use tonic::transport::Server;

use crate::matching_engine::MatchingEngine;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let database = PgPool::connect(std::env::var("DATABASE_URL")?.as_str()).await?;

    let matching_engine = MatchingEngine::new(
        database,
        std::env::var("ENGINE_FRACTION_ACCURACY")?.parse()?,
    );

    let addr = SocketAddr::from(([0, 0, 0, 0], 80));
    tracing::info!("listening on {}", addr);

    Server::builder()
        .add_service(EngineServer::new(matching_engine))
        .serve_with_shutdown(addr, shutdown_signal::listen())
        .await?;

    Ok(())
}
