pub use engine::matching_engine::models::{burn, mint};
use ethers::{
    prelude::*,
    providers::{Provider, Ws},
};
use fraction::Fraction;
use sqlx::PgPool;
use tokio::{
    select,
    sync::{mpsc, oneshot},
};
use tonic::{transport::Channel, Status};

use crate::{
    confirmation::ConfirmationQueue,
    contracts::treasury::{Treasury, TreasuryEvents},
    database::{
        managers::{
            assets::AssetsManager, deposits::DepositsManager,
            notification::NotificationManagerOutput, users::UsersManager,
            withdraws::WithdrawsManager,
        },
        projections::{
            deposit::{Deposit, DepositInsert},
            withdraw::{Withdraw, WithdrawInsert},
        },
    },
    engine_base::{engine_client::EngineClient, BurnRequest, MintRequest},
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

    pub async fn start(
        mut self,
    ) -> Result<BlockchainManagerController, ContractError<Provider<Ws>>> {
        let (shutdown_tx, mut shutdown_rx) = oneshot::channel::<()>();
        let join_handle_confirmation: tokio::task::JoinHandle<Result<(), BlockchainManagerError>> =
            tokio::spawn(async move {
                let contract_events = self.contract.events();
                let deposits_manager = DepositsManager::new(self.database.to_owned());
                let withdraws_manager = WithdrawsManager::new(self.database.to_owned());
                let users_manager = UsersManager::new(self.database.to_owned());
                let assets_manager = AssetsManager::new(self.database.to_owned());
                let mut blocks_stream = self.provider.subscribe_blocks().await?;
                let mut events_stream = contract_events.stream_with_meta().await?;

                let mut deposits_queue = ConfirmationQueue::<Deposit>::new(&self.provider);
                let mut withdraws_queue = ConfirmationQueue::<Withdraw>::new(&self.provider);

                loop {
                    select! {
                        _ = &mut shutdown_rx => {
                            break;
                        },
                        Some(log) = events_stream.next() => {
                            let (event, meta) = log?;
                            match event {
                                TreasuryEvents::DepositFilter(event) => {
                                    let asset = assets_manager.get_by_address(event.token_address.into()).await?;
                                    let mut bytes = [0_u8; 32];
                                    event.amount.to_little_endian(&mut bytes);
                                    deposits_queue.insert(deposits_manager.insert(DepositInsert {
                                        user_id: users_manager.get_by_address(event.user_address.into()).await?.id,
                                        asset_id: asset.id,
                                        amount: Fraction::from_bytes_le(&bytes) / asset.decimals,
                                        tx_hash: meta.transaction_hash.into(),
                                        confirmations: Fraction::default()
                                    }).await?, meta.transaction_hash).await?;
                                }
                                TreasuryEvents::WithdrawFilter(event) => {
                                    let asset = assets_manager.get_by_address(event.token_address.into()).await?;
                                    let mut bytes = [0_u8; 32];
                                    event.amount.to_little_endian(&mut bytes);
                                    withdraws_queue.insert(withdraws_manager.insert(WithdrawInsert {
                                        user_id: users_manager.get_by_address(event.user_address.into()).await?.id,
                                        asset_id: asset.id,
                                        amount: Fraction::from_bytes_le(&bytes) / asset.decimals,
                                        tx_hash: meta.transaction_hash.into(),
                                        confirmations: Fraction::default()
                                    }).await?, meta.transaction_hash).await?;
                                }
                                _ => (),
                            }
                        },
                        Some(block) = blocks_stream.next() => {
                            for deposit in deposits_queue.confirmation_step(&block).await?.into_iter() {
                                self.engine_client.mint(TryInto::<MintRequest>::try_into(deposit)?).await?;
                            }

                            for withdraw in withdraws_queue.confirmation_step(&block).await?.into_iter() {
                                self.engine_client.burn(TryInto::<BurnRequest>::try_into(withdraw)?).await?;
                            }
                        }
                    }
                }

                Ok(())
            });
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

    #[error(transparent)]
    Status(#[from] Status),

    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
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
