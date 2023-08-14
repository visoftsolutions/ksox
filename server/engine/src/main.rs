pub mod base {
    tonic::include_proto!("server.engine.base");
}

mod database;
mod matching_engine;
mod shutdown_signal;

use base::engine_server::EngineServer;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tonic::transport::Server;

use crate::matching_engine::MatchingEngine;

use macros::retry;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let database = retry!(PgPoolOptions::new()
        .max_connections(10)
        .connect(std::env::var("KSOX_POSTGRES_URL").unwrap().as_str()))?;

    let matching_engine = MatchingEngine::new(database);

    let addr = SocketAddr::from(([0, 0, 0, 0], 80));
    tracing::info!("listening on {}", addr);

    Server::builder()
        .add_service(EngineServer::new(matching_engine))
        .serve_with_shutdown(addr, shutdown_signal::listen())
        .await?;

    Ok(())
}
