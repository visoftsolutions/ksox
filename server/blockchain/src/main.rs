pub mod blockchain_engine;
pub mod confirmation;
pub mod contracts;
pub mod database;
pub mod models;
pub mod submission;
pub mod timeout;

mod shutdown_signal;
use contracts::treasury::Treasury;
use database::managers::notification::NotificationManager;
use engine_base::engine_client::EngineClient;
use ethers::{
    prelude::SignerMiddleware,
    providers::{Provider, Ws},
    signers::LocalWallet,
};
use sqlx::postgres::PgPoolOptions;
use std::io;
use std::net::SocketAddr;
use tonic::transport::Server;

use crate::{
    base::blockchain_server::BlockchainServer,
    blockchain_engine::{
        deposits::DepositsBlockchainManager, valuts::ValutsBlockchainManager,
        withdraws::WithdrawsBlockchainManager, BlockchainEngine,
    },
};

pub mod engine_base {
    tonic::include_proto!("server.engine.base");
}

pub mod base {
    tonic::include_proto!("server.blockchain.base");
}

use macros::retry;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let database = retry!(PgPoolOptions::new()
        .max_connections(10)
        .connect(std::env::var("DATABASE_URL").unwrap().as_str()))?;
    let engine_client = retry!(EngineClient::connect(std::env::var("ENGINE_URL").unwrap()))?;
    let provider = retry!(Provider::<Ws>::connect(
        std::env::var("WS_PROVIDER_URL").unwrap()
    ))?;

    let notification_manager_controller =
        NotificationManager::start(database.clone(), "blockchain").await?;

    let wallet: LocalWallet = std::env::var("PRIVATE_KEY")?.parse()?;
    let client = SignerMiddleware::new_with_provider_chain(provider.clone(), wallet).await?;
    let treasury = Treasury::new(
        prefix_hex::decode::<[u8; 20]>(std::env::var("TREASURY_ADDRESS")?)
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err.to_string()))?,
        std::sync::Arc::new(provider.clone()),
    )
    .connect(client.into());

    let deposits_controller = DepositsBlockchainManager {
        database: database.to_owned(),
        provider: provider.to_owned(),
        contract: treasury.to_owned().into(),
    }
    .start(engine_client.to_owned())
    .await;

    let withdraws_controller = WithdrawsBlockchainManager {
        database: database.to_owned(),
        provider: provider.to_owned(),
        contract: treasury.to_owned().into(),
    }
    .start(engine_client.to_owned())
    .await;

    let valuts_controller = ValutsBlockchainManager {
        database: database.to_owned(),
        provider,
        contract: treasury.to_owned().into(),
    }
    .start(
        notification_manager_controller
            .get_subscriber()
            .subscribe_to()
            .await?,
    )
    .await;

    let engagement = BlockchainEngine {
        contract: treasury.into(),
        database: database.to_owned(),
        deposits_controller,
        withdraws_controller,
        valuts_controller,
    };
    let addr = SocketAddr::from(([0, 0, 0, 0], 80));
    tracing::info!("listening on {}", addr);

    Server::builder()
        .add_service(BlockchainServer::new(engagement))
        .serve_with_shutdown(addr, shutdown_signal::listen())
        .await?;

    Ok(())
}
