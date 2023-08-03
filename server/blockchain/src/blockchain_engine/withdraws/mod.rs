pub mod models;

use ethers::{
    prelude::{k256::ecdsa::SigningKey, SignerMiddleware},
    providers::{Middleware, Provider, Ws},
    signers::Wallet,
};

use sqlx::PgPool;
use tokio::{
    select,
    sync::{mpsc, oneshot},
    task::JoinError,
};
use tokio_stream::StreamExt;
use tonic::{transport::Channel, Status};

use crate::{
    contracts::treasury::{Treasury, TreasuryEvents, WithdrawFilter},
    database::{
        managers::withdraws::WithdrawsManager,
        projections::withdraw::{withdraw_to_transfer_request, WithdrawInsert},
    },
    engine_base::{engine_client::EngineClient, RevertTransferRequest},
    models::BlockchainManagerError,
};

pub struct WithdrawsBlockchainManager {
    pub database: PgPool,
    pub provider: Provider<Ws>,
    pub contract: Treasury<SignerMiddleware<Provider<Ws>, Wallet<SigningKey>>>,
}

impl WithdrawsBlockchainManager {
    pub async fn start(
        &self,
        mut engine_client: EngineClient<Channel>,
    ) -> WithdrawsBlockchainManagerController {
        let (shutdown_tx, mut shutdown_rx) = oneshot::channel::<()>();
        let (insert_withdraw_event_tx, mut insert_withdraw_event_rx) =
            mpsc::channel::<WithdrawInsert>(100);

        let database = self.database.to_owned();
        let provider = self.provider.to_owned();
        let contract = self.contract.to_owned();
        let contract_events = self.contract.events();
        let join_handle: tokio::task::JoinHandle<Result<(), BlockchainManagerError>> = tokio::spawn(
            async move {
                let mut blocks_stream = provider.subscribe_blocks().await?;
                let mut events_stream = contract_events.stream_with_meta().await?;
                // let mut timeout_queue = TimeoutQueue::new(Duration::from_secs(3600));

                loop {
                    select! {
                        _ = &mut shutdown_rx => {
                            break;
                        },
                        Some(event) = insert_withdraw_event_rx.recv() => {
                            if let Err(err) = async {
                                let mut t = database.begin().await.map_err(|e| Status::aborted(e.to_string()))?;
                                let withdraw = WithdrawsManager::insert(&mut t, &event).await?;
                                let filter: WithdrawFilter = event.to_filter(&mut t).await?;
                                let response = engine_client.transfer(withdraw_to_transfer_request(&mut t, withdraw).await?).await?.into_inner();
                                if let Err(err) = async {
                                     contract.withdraw(filter.token_address, filter.user_address, filter.amount).send().await?;
                                    Ok::<(), BlockchainManagerError>(())
                                }.await {
                                    engine_client.revert_transfer(Into::<RevertTransferRequest>::into(response)).await?;
                                    Err(err)
                                } else {
                                    Ok(t.commit().await?)
                                }
                            }.await {
                                tracing::error!("{err}");
                            }
                        },
                        Some(Ok((filter, meta))) = events_stream.next() => {
                            if let Err(err) = async {
                                let mut t = database.begin().await.map_err(|e| Status::aborted(e.to_string()))?;
                                if let TreasuryEvents::WithdrawFilter(filter) = filter {
                                    let _event = WithdrawInsert::from_filter(&mut t, &filter, &meta).await?;
                                };
                                t.commit().await?;
                                Ok::<(), BlockchainManagerError>(())
                            }.await {
                                tracing::error!("{err}");
                            }
                        },
                        Some(_block) = blocks_stream.next() => {
                            if let Err(err) = async {
                                let t = database.begin().await.map_err(|e| Status::aborted(e.to_string()))?;
                                t.commit().await?;
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
        WithdrawsBlockchainManagerController {
            shutdown_tx,
            join_handle,
            insert_withdraw_event_tx,
        }
    }
}

#[derive(Debug)]
pub struct WithdrawsBlockchainManagerController {
    shutdown_tx: oneshot::Sender<()>,
    insert_withdraw_event_tx: mpsc::Sender<WithdrawInsert>,
    join_handle: tokio::task::JoinHandle<Result<(), BlockchainManagerError>>,
}
impl WithdrawsBlockchainManagerController {
    pub async fn shutdown(self) -> Result<Result<(), BlockchainManagerError>, JoinError> {
        if self.shutdown_tx.send(()).is_err() {
            tracing::error!("Error: shutdown");
        }
        self.join_handle.await
    }

    pub async fn withdraw(
        &self,
        request: WithdrawInsert,
    ) -> Result<(), mpsc::error::SendError<WithdrawInsert>> {
        self.insert_withdraw_event_tx.send(request).await
    }
}
