use std::pin::Pin;

pub use engine::matching_engine::models::{burn, mint};
use ethers::{
    prelude::*,
    providers::{Provider, Ws},
};
use evm::txhash::TxHash;
use fraction::Fraction;
use futures::Future;
use sqlx::PgPool;
use tokio::{
    select,
    sync::{mpsc, oneshot},
    task::JoinError,
};
use tonic::{transport::Channel, Status};

use crate::{
    confirmation::ConfirmationQueue,
    contracts::treasury::{DepositFilter, Treasury, TreasuryEvents, WithdrawFilter},
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

    pub async fn start_confirmation(
        &self,
    ) -> ConfirmationManagerController {
        let (shutdown_tx, mut shutdown_rx) = oneshot::channel::<()>();
        let (insert_deposit_event_tx, mut insert_deposit_event_rx) =
            mpsc::channel::<(DepositFilter, TxHash)>(100);
        let (insert_withdraw_event_tx, mut insert_withdraw_event_rx) =
            mpsc::channel::<(WithdrawFilter, TxHash)>(100);

        let database = self.database.to_owned();
        let provider = self.provider.to_owned();
        let contract_events = self.contract.events();
        let deposits_manager = DepositsManager::new(self.database.to_owned());
        let withdraws_manager = WithdrawsManager::new(self.database.to_owned());
        let users_manager = UsersManager::new(self.database.to_owned());
        let assets_manager = AssetsManager::new(self.database.to_owned());
        let mut engine_client = self.engine_client.to_owned();
        let join_handle: tokio::task::JoinHandle<Result<(), BlockchainManagerError>> = tokio::spawn(
            async move {
                let mut blocks_stream = provider.subscribe_blocks().await?;
                let mut events_stream = contract_events.stream_with_meta().await?;
                let mut deposits_queue = ConfirmationQueue::<Deposit>::new(&provider);
                let mut withdraws_queue = ConfirmationQueue::<Withdraw>::new(&provider);
                let deposit_event_to_insert = |event: DepositFilter,
                                               tx_hash: TxHash|
                 -> Pin<
                    Box<dyn Future<Output = Result<DepositInsert, sqlx::Error>> + Send>,
                > {
                    let asset = assets_manager.get_by_address(event.token_address.into());
                    let user = users_manager.get_by_address(event.user_address.into());
                    Box::pin(async move {
                        let asset = asset.await?;
                        let user = user.await?;
                        let mut bytes = [0_u8; 32];
                        event.amount.to_little_endian(&mut bytes);
                        Ok(DepositInsert {
                            user_id: user.id,
                            asset_id: asset.id,
                            amount: Fraction::from_bytes_le(&bytes) / asset.decimals,
                            tx_hash: tx_hash.into(),
                            confirmations: Fraction::default(),
                        })
                    })
                };
                let withdraw_event_to_insert = |event: WithdrawFilter,
                                                tx_hash: TxHash|
                 -> Pin<
                    Box<dyn Future<Output = Result<WithdrawInsert, sqlx::Error>> + Send>,
                > {
                    let asset = assets_manager.get_by_address(event.token_address.into());
                    let user = users_manager.get_by_address(event.user_address.into());
                    Box::pin(async move {
                        let asset = asset.await?;
                        let user = user.await?;
                        let mut bytes = [0_u8; 32];
                        event.amount.to_little_endian(&mut bytes);
                        Ok(WithdrawInsert {
                            user_id: user.id,
                            asset_id: asset.id,
                            amount: Fraction::from_bytes_le(&bytes) / asset.decimals,
                            tx_hash,
                            confirmations: Fraction::default(),
                        })
                    })
                };
                loop {
                    select! {
                        _ = &mut shutdown_rx => {
                            break;
                        },
                        Some((event, tx_hash)) = insert_deposit_event_rx.recv() => {
                            if let Err(err) = async {
                                let insert = deposit_event_to_insert(event, tx_hash.clone()).await?;
                                let deposit = deposits_manager.insert(insert).await?;
                                Ok::<(), BlockchainManagerError>(deposits_queue.insert(deposit, *tx_hash).await?)
                            }.await {
                                tracing::error!("{err}");
                            }
                        },
                        Some((event, tx_hash)) = insert_withdraw_event_rx.recv() => {
                            if let Err(err) = async {
                                let insert = withdraw_event_to_insert(event, tx_hash.clone()).await?;
                                let deposit = withdraws_manager.insert(insert).await?;
                                Ok::<(), BlockchainManagerError>(withdraws_queue.insert(deposit, *tx_hash).await?)
                            }.await {
                                tracing::error!("{err}");
                            }
                        },
                        Some(Ok((event, meta))) = events_stream.next() => {
                            if let Err(err) = async {
                                Ok::<(), BlockchainManagerError>(
                                    match event {
                                        TreasuryEvents::DepositFilter(event) => {
                                            let insert = deposit_event_to_insert(event, meta.transaction_hash.into()).await?;
                                            let deposit = deposits_manager.insert(insert).await?;
                                            deposits_queue.insert(deposit, meta.transaction_hash).await?;
                                        }
                                        TreasuryEvents::WithdrawFilter(event) => {
                                            let insert = withdraw_event_to_insert(event, meta.transaction_hash.into()).await?;
                                            let withdraw = withdraws_manager.insert(insert).await?;
                                            withdraws_queue.insert(withdraw, meta.transaction_hash).await?;
                                        }
                                        _ => (),
                                    }
                                )
                            }.await {
                                tracing::error!("{err}");
                            }
                        },
                        Some(block) = blocks_stream.next() => {
                            if let Err(err) = async {
                                let mut t = database.begin().await.map_err(|e| Status::aborted(e.to_string()))?;
                                let (confirmed_deposit, not_confirmed_deposits) = deposits_queue.confirmation_step(&block).await?;
                                let (confirmed_withdraw, not_confirmed_withdraw) = withdraws_queue.confirmation_step(&block).await?;
                                for deposit in not_confirmed_deposits.into_iter() {
                                    deposits_manager.update(&mut t, deposit).await?;
                                }
                                for deposit in confirmed_deposit.iter().cloned() {
                                    deposits_manager.update(&mut t, deposit).await?;
                                }
                                for withdraw in not_confirmed_withdraw.into_iter() {
                                    withdraws_manager.update(&mut t, withdraw).await?;
                                }
                                for withdraw in confirmed_withdraw.iter().cloned() {
                                    withdraws_manager.update(&mut t, withdraw).await?;
                                }
                                for deposit in confirmed_deposit.into_iter() {
                                    engine_client.mint(TryInto::<MintRequest>::try_into(deposit)?).await?;
                                }
                                for withdraw in confirmed_withdraw.into_iter() {
                                    engine_client.burn(TryInto::<BurnRequest>::try_into(withdraw)?).await?;
                                }
                                Ok::<(), BlockchainManagerError>(t.commit().await?)
                            }.await {
                                tracing::error!("{err}");
                            }
                        }
                    }
                }
                Ok(())
            },
        );
        ConfirmationManagerController {
            shutdown_tx,
            join_handle,
            insert_deposit_event_tx,
            insert_withdraw_event_tx,
        }
    }

    pub async fn start_submission(
        &self,
    ) -> SubmissionManagerController {

        todo!()
    }
}

#[derive(Debug)]
pub struct ConfirmationManagerController {
    shutdown_tx: oneshot::Sender<()>,
    insert_deposit_event_tx: mpsc::Sender<(DepositFilter, TxHash)>,
    insert_withdraw_event_tx: mpsc::Sender<(WithdrawFilter, TxHash)>,
    join_handle: tokio::task::JoinHandle<Result<(), BlockchainManagerError>>,
}
impl ConfirmationManagerController {
    pub async fn shutdown(self) -> Result<Result<(), BlockchainManagerError>, JoinError> {
        if self.shutdown_tx.send(()).is_err() {
            tracing::error!("Error: shutdown");
        }
        Ok(self.join_handle.await?)
    }

    pub async fn submit_deposit_event(
        &self,
        event: (DepositFilter, TxHash),
    ) -> Result<(), mpsc::error::SendError<(DepositFilter, TxHash)>> {
        self.insert_deposit_event_tx.send(event).await
    }

    pub async fn submit_withdraw_event(
        &self,
        event: (WithdrawFilter, TxHash),
    ) -> Result<(), mpsc::error::SendError<(WithdrawFilter, TxHash)>> {
        self.insert_withdraw_event_tx.send(event).await
    }
}

#[derive(Debug)]
pub struct SubmissionManagerController {
    shutdown_tx: oneshot::Sender<()>,
    join_handle: tokio::task::JoinHandle<Result<(), BlockchainManagerError>>,
}
impl SubmissionManagerController {
    pub async fn shutdown(self) -> Result<Result<(), BlockchainManagerError>, JoinError> {
        if self.shutdown_tx.send(()).is_err() {
            tracing::error!("Error: shutdown");
        }
        Ok(self.join_handle.await?)
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
