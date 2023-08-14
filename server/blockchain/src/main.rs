pub mod blockchain_engine;
pub mod contracts;
pub mod database;
pub mod models;

mod shutdown_signal;
use contracts::treasury::Treasury;
use engine_base::engine_client::EngineClient;
use ethers::{
    providers::{Provider, Ws},
    signers::{LocalWallet, Signer},
};
use evm::address::Address;
use num_bigint::BigInt;
use sqlx::postgres::PgPoolOptions;
use std::{io, str::FromStr};
use std::net::SocketAddr;
use tonic::transport::Server;

use crate::{
    base::blockchain_server::BlockchainServer,
    blockchain_engine::{
        deposits::DepositsBlockchainManager, withdraws::WithdrawsBlockchainManager,
        BlockchainEngine,
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

    let contract_key_wallet: LocalWallet = std::env::var("CONTRACT_PRIVATE_KEY")?.parse()?;

    let treasury = Treasury::new(
        prefix_hex::decode::<[u8; 20]>(std::env::var("CONTRACT_ADDRESS")?)
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err.to_string()))?,
        std::sync::Arc::new(provider.clone()),
    );

    let confirmations = BigInt::from_str(std::env::var("KSOX_SERVER_DEPOSIT_CONFIRMATIONS").unwrap().as_str())?;

    let deposits_controller = DepositsBlockchainManager {
        database: database.to_owned(),
        provider: provider.to_owned(),
        contract: treasury.to_owned(),
        confirmations: confirmations.to_owned(),
    }
    .start(engine_client.to_owned())
    .await;

    let withdraws_controller = WithdrawsBlockchainManager {
        database: database.to_owned(),
        provider: provider.to_owned(),
        contract: treasury.to_owned(),
        contract_key_wallet,
    }
    .start(engine_client.to_owned())
    .await;

    let blockchain_engine = BlockchainEngine {
        deposits_controller,
        withdraws_controller,
        engine_client,
    };
    let addr = SocketAddr::from(([0, 0, 0, 0], 80));
    tracing::info!("listening on {}", addr);

    Server::builder()
        .add_service(BlockchainServer::new(blockchain_engine))
        .serve_with_shutdown(addr, shutdown_signal::listen())
        .await?;

    Ok(())
}
