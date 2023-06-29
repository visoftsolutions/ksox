pub use engine::matching_engine::models::{burn, mint};
use ethers::{
    prelude::*,
    providers::{Provider, Ws},
};
use sqlx::PgPool;
use tokio::sync::{mpsc, oneshot};
use tonic::transport::Channel;

use crate::{
    confirmation::ConfirmationQueue,
    contracts::treasury::{DepositFilter, Treasury, TreasuryEvents, WithdrawFilter},
    database::{
        managers::{
            deposits::DepositsManager, notification::NotificationManagerOutput,
            withdraws::WithdrawsManager,
        },
        projections::{deposit::Deposit, withdraw::Withdraw},
    },
    engine_base::engine_client::EngineClient,
};

pub struct BlockchainManager {
    database: PgPool,
    provider: Provider<Ws>,
    contract: Treasury<Provider<Ws>>,
    engine_client: EngineClient<Channel>,
    notifications: mpsc::Receiver<NotificationManagerOutput>,
}

impl BlockchainManager {
    pub fn new(
        database: PgPool,
        provider: Provider<Ws>,
        contract: Treasury<Provider<Ws>>,
        engine_client: EngineClient<Channel>,
        notifications: mpsc::Receiver<NotificationManagerOutput>,
    ) -> Self {
        Self {
            database,
            provider,
            contract,
            engine_client,
            notifications,
        }
    }

    pub async fn start(self) -> Result<BlockchainManagerController, ContractError<Provider<Ws>>> {
        let join_handle: tokio::task::JoinHandle<Result<(), BlockchainManagerError>> =
            tokio::spawn(async move {
                let contract_events = self.contract.events();
                let deposits_manager = DepositsManager::new();
                let withdraws_manager = WithdrawsManager::new();
                let mut contract_events_stream = contract_events.stream_with_meta().await?;

                let mut deposits_queue = ConfirmationQueue::<_, DepositFilter>::new(
                    &self.database,
                    &deposits_manager,
                    &self.provider,
                );
                let mut withdraws_queue = ConfirmationQueue::<_, WithdrawFilter>::new(
                    &self.database,
                    &withdraws_manager,
                    &self.provider,
                );

                while let Some(Ok((event, meta))) = contract_events_stream.next().await {
                    match event {
                        TreasuryEvents::DepositFilter(event) => {
                            deposits_queue.insert(event, meta.transaction_hash).await?;
                        }
                        TreasuryEvents::WithdrawFilter(event) => {
                            withdraws_queue.insert(event, meta.transaction_hash).await?;
                        }
                        _ => (),
                    }
                }

                Ok(())
            });

        // monitor deposits and witdraws on smartocntracts events and tx hashes
        // monitor changes on valuts  trigger is
        todo!()
    }
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum BlockchainManagerError {
    #[error(transparent)]
    ContractError(#[from] ContractError<Provider<Ws>>),

    #[error(transparent)]
    ProviderError(#[from] ProviderError),
}

#[derive(Debug)]
pub struct BlockchainManagerController {
    shutdown_tx: oneshot::Sender<()>,
    join_handle: tokio::task::JoinHandle<Result<(), BlockchainManagerError>>,
}
impl BlockchainManagerController {
    pub async fn shutdown(self) -> Result<(), tokio::task::JoinError> {
        if self.shutdown_tx.send(()).is_err() {
            tracing::error!("Error: shutdown");
        }
        self.join_handle.await?;
        Ok(())
    }
}

pub struct FlowMonitor {}

impl FlowMonitor {
    pub fn start() {
        let (tx, rx) = oneshot::channel::<()>();
        let join_handle = tokio::spawn(async move {});
    }
}

pub struct FlowMonitorController {
    shutdown_tx: oneshot::Sender<()>,
    join_handle: tokio::task::JoinHandle<()>,
}

impl FlowMonitorController {
    pub async fn shutdown(self) -> Result<(), tokio::task::JoinError> {
        if self.shutdown_tx.send(()).is_err() {
            tracing::error!("Error: shutdown");
        }
        self.join_handle.await?;
        Ok(())
    }
}
