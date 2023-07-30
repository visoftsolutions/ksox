pub mod models;
use ethers::{
    prelude::{k256::ecdsa::SigningKey, SignerMiddleware},
    providers::{Middleware, Provider, Ws},
    signers::Wallet,
};
use futures::stream::StreamExt;
use sqlx::PgPool;
use tokio::{
    select,
    sync::{mpsc, oneshot},
    task::JoinError,
};
use tonic::{transport::Channel, Status};

use crate::{
    confirmation::ConfirmationQueue,
    contracts::treasury::{Treasury, TreasuryEvents},
    database::{
        managers::deposits::DepositsManager,
        projections::deposit::{deposit_to_transfer_request, Deposit, DepositInsert},
    },
    engine_base::{self, engine_client::EngineClient},
    models::BlockchainManagerError,
};

pub struct DepositsBlockchainManager {
    pub database: PgPool,
    pub provider: Provider<Ws>,
    pub contract: Treasury<SignerMiddleware<Provider<Ws>, Wallet<SigningKey>>>,
}

impl DepositsBlockchainManager {
    pub async fn start(
        &self,
        mut engine_client: EngineClient<Channel>,
    ) -> DepositsBlockchainManagerController {
        let (shutdown_tx, mut shutdown_rx) = oneshot::channel::<()>();
        let (insert_deposit_event_tx, mut insert_deposit_event_rx) =
            mpsc::channel::<DepositInsert>(100);

        let database = self.database.to_owned();
        let provider = self.provider.to_owned();
        let contract_events = self.contract.events();
        let join_handle: tokio::task::JoinHandle<Result<(), BlockchainManagerError>> = tokio::spawn(
            async move {
                let mut blocks_stream = provider.subscribe_blocks().await?;
                let mut events_stream = contract_events.stream_with_meta().await?;
                let mut deposits_queue = ConfirmationQueue::<Deposit>::new(&provider);

                loop {
                    select! {
                        _ = &mut shutdown_rx => {
                            break;
                        },
                        Some(event) = insert_deposit_event_rx.recv() => {
                            if let Err(err) = async {
                                let mut t = database.begin().await.map_err(|e| Status::aborted(e.to_string()))?;
                                let deposit = DepositsManager::insert(&mut t, &event).await?;
                                deposits_queue.insert(deposit, *event.tx_hash).await?;
                                t.commit().await?;
                                Ok::<(), BlockchainManagerError>(())
                            }.await {
                                tracing::error!("{err}");
                            }
                        },
                        Some(Ok((filter, meta))) = events_stream.next() => {
                            if let Err(err) = async {
                                let mut t = database.begin().await.map_err(|e| Status::aborted(e.to_string()))?;
                                if let TreasuryEvents::DepositFilter(filter) = filter {
                                        let insert = DepositInsert::from_filter(&mut t, &filter, &meta).await?;
                                        let deposit = DepositsManager::insert(&mut t, &insert).await?;
                                        deposits_queue.insert(deposit, meta.transaction_hash).await?;
                                };
                                t.commit().await?;
                                Ok::<(), BlockchainManagerError>(())
                            }.await {
                                tracing::error!("{err}");
                            }
                        },
                        Some(block) = blocks_stream.next() => {
                            if let Err(err) = async {
                                let mut t = database.begin().await.map_err(|e| Status::aborted(e.to_string()))?;
                                let (confirmed_deposit, not_confirmed_deposits) = deposits_queue.confirmation_step(&block).await;
                                for deposit in not_confirmed_deposits.into_iter() {
                                    DepositsManager::update(&mut t, deposit).await?;
                                }
                                for deposit in confirmed_deposit.iter().cloned() {
                                    DepositsManager::update(&mut t, deposit).await?;
                                }

                                let mut transfers: Vec<engine_base::TransferRequest> = Vec::new();

                                for deposit in confirmed_deposit {
                                    transfers.push(deposit_to_transfer_request(&mut t, deposit).await?);
                                }

                                t.commit().await?;

                                for transfer in transfers.into_iter() {
                                    engine_client.transfer(transfer).await?;
                                }
                                Ok::<(), BlockchainManagerError>(())
                            }.await {
                                tracing::error!("{err}");
                            }
                        }
                    }
                }
                Ok(())
            },
        );
        DepositsBlockchainManagerController {
            shutdown_tx,
            join_handle,
            insert_deposit_event_tx,
        }
    }
}

#[derive(Debug)]
pub struct DepositsBlockchainManagerController {
    shutdown_tx: oneshot::Sender<()>,
    insert_deposit_event_tx: mpsc::Sender<DepositInsert>,
    join_handle: tokio::task::JoinHandle<Result<(), BlockchainManagerError>>,
}
impl DepositsBlockchainManagerController {
    pub async fn shutdown(self) -> Result<Result<(), BlockchainManagerError>, JoinError> {
        if self.shutdown_tx.send(()).is_err() {
            tracing::error!("Error: shutdown");
        }
        self.join_handle.await
    }

    pub async fn submit_deposit_event(
        &self,
        event: DepositInsert,
    ) -> Result<(), mpsc::error::SendError<DepositInsert>> {
        self.insert_deposit_event_tx.send(event).await
    }
}
