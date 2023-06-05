#![feature(let_chains)]

pub mod base {
    tonic::include_proto!("server.engagement.base");
}

mod database;
mod engagement_engine;
mod shutdown_signal;

use std::net::SocketAddr;

use base::engagement_server::EngagementServer;
use sqlx::PgPool;
use tonic::transport::Server;

use crate::engagement_engine::EngagementEngine;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let database = PgPool::connect(std::env::var("DATABASE_URL")?.as_str()).await?;

    let engagement = EngagementEngine::new(
        database
    );

    let addr = SocketAddr::from(([0, 0, 0, 0], 80));
    tracing::info!("listening on {}", addr);

    Server::builder()
        .add_service(EngagementServer::new(engagement))
        .serve_with_shutdown(addr, shutdown_signal::listen())
        .await?;

    Ok(())
}
