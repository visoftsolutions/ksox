pub mod base {
    tonic::include_proto!("server.engagement.base");
}

pub mod database;
pub mod engagement_engine;
mod shutdown_signal;

use base::engagement_server::EngagementServer;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tonic::transport::Server;

use crate::{
    database::managers::notifications::NotificationManager, engagement_engine::EngagementEngine,
};

use macros::retry;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let database = retry!(PgPoolOptions::new()
        .max_connections(10)
        .connect(std::env::var("DATABASE_URL").unwrap().as_str()))?;

    let notification_manager_controller =
        NotificationManager::start(database, "engagement").await?;

    let engagement = EngagementEngine::default();

    let addr = SocketAddr::from(([0, 0, 0, 0], 80));
    tracing::info!("listening on {}", addr);

    Server::builder()
        .add_service(EngagementServer::new(engagement))
        .serve_with_shutdown(addr, shutdown_signal::listen())
        .await?;

    notification_manager_controller.shutdown().await?;

    Ok(())
}
