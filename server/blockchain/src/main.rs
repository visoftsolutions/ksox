pub mod confirmation;
pub mod contracts;
pub mod database;
pub mod models;
pub mod submission;

mod shutdown_signal;
use std::net::SocketAddr;
use std::io;
use tonic::transport::Server;
use contracts::treasury::Treasury;
use database::managers::notification::NotificationManager;
use engine_base::engine_client::EngineClient;
use ethers::providers::{Provider, Ws};
use models::BlockchainManager;
use sqlx::postgres::PgPoolOptions;

pub mod engine_base {
    tonic::include_proto!("server.engine.base");
}

pub mod base {
    tonic::include_proto!("server.blockchain.base");
}

use crate::base::blockchain_server::{Blockchain, BlockchainServer};

#[derive(Debug, Default)]
pub struct BlockchainEngine {}

#[tonic::async_trait]
impl Blockchain for BlockchainEngine {}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let database = PgPoolOptions::new()
        .max_connections(10)
        .connect(std::env::var("DATABASE_URL")?.as_str())
        .await?;
    let engine_client = EngineClient::connect(std::env::var("ENGINE_URL")?).await?;
    let provider = Provider::<Ws>::connect(std::env::var("WS_PROVIDER_URL")?).await?;
    let notification_manager_controller =
        NotificationManager::start(database.clone(), "blockchain").await?;
    let treasury = Treasury::new(
        prefix_hex::decode::<[u8; 20]>(std::env::var("TREASURY_ADDRESS")?)
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err.to_string()))?,
        std::sync::Arc::new(provider.clone()),
    );

    let blockchain_manager = BlockchainManager::new(database, provider, treasury);

    let confirmation_controller = blockchain_manager.start_confirmation(engine_client).await;

    let submission_controller = blockchain_manager
        .start_submission(
            notification_manager_controller
                .get_subscriber()
                .subscribe_to()
                .await?,
        )
        .await;

    let engagement = BlockchainEngine::default();
    let addr = SocketAddr::from(([0, 0, 0, 0], 80));
    tracing::info!("listening on {}", addr);
    
    Server::builder()
        .add_service(BlockchainServer::new(engagement))
        .serve_with_shutdown(addr, shutdown_signal::listen())
        .await?;
    

    if let Err(err) = confirmation_controller.shutdown().await? {
        tracing::error!("{err}");
    }

    if let Err(err) = submission_controller.shutdown().await? {
        tracing::error!("{err}");
    }

    Ok(())
}
