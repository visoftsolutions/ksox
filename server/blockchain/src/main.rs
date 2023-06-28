#![feature(async_fn_in_trait)]
#![feature(const_trait_impl)]

pub mod confirmation;
pub mod contracts;
pub mod database;
pub mod models;
pub mod submission;

use engine_base::engine_client::EngineClient;
use sqlx::postgres::PgPoolOptions;

pub mod engine_base {
    tonic::include_proto!("server.engine.base");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let BLOCK_CONFIRMATIONS: usize = std::env::var("WORKER_FRACTION_ACCURACY")?.parse()?;
    let database = PgPoolOptions::new()
        .max_connections(10)
        .connect(std::env::var("DATABASE_URL")?.as_str())
        .await?;
    let engine_client = EngineClient::connect(std::env::var("ENGINE_URL")?).await?;
    Ok(())
}
