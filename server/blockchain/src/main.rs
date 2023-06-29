#![feature(async_fn_in_trait)]
#![feature(const_trait_impl)]

pub mod confirmation;
pub mod contracts;
pub mod database;
pub mod models;
pub mod submission;

use std::io;

use contracts::treasury::Treasury;
use database::managers::notification::NotificationManager;
use engine_base::engine_client::EngineClient;
use ethers::providers::{Http, Provider, Ws};
use sqlx::postgres::PgPoolOptions;

pub mod engine_base {
    tonic::include_proto!("server.engine.base");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let BLOCK_CONFIRMATIONS: usize = std::env::var("BLOCK_CONFIRMATIONS")?.parse()?;
    let database = PgPoolOptions::new()
        .max_connections(10)
        .connect(std::env::var("DATABASE_URL")?.as_str())
        .await?;
    let engine_client = EngineClient::connect(std::env::var("ENGINE_URL")?).await?;

    let http_provider = Provider::<Http>::try_from(std::env::var("HTTP_PROVIDER_URL")?)?;
    let ws_provider = Provider::<Ws>::connect(std::env::var("WS_PROVIDER_URL")?).await?;

    let notification_manager_controller =
        NotificationManager::start(database, "notifications").await?;

    let treasury = Treasury::new(
        prefix_hex::decode::<[u8; 20]>(std::env::var("TREASURY_ADDRESS")?)
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err.to_string()))?,
        std::sync::Arc::new(ws_provider),
    );
    Ok(())
}
