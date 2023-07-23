pub mod models;

use ethers::{
    prelude::{k256::ecdsa::SigningKey, SignerMiddleware},
    providers::{Middleware, Provider, Ws},
    signers::Wallet,
};
use evm::txhash::TxHash;
use sqlx::PgPool;
use tokio::{
    select,
    sync::{mpsc, oneshot},
    task::JoinError,
};
use tokio_stream::StreamExt;
use tonic::{transport::Channel, Status};

use crate::{
    contracts::treasury::{Treasury, TreasuryEvents},
    engine_base::engine_client::EngineClient,
    models::BlockchainManagerError,
};

use self::models::{WithdrawEvent, WithdrawRequest};

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
        let (insert_withdraw_request_tx, mut insert_withdraw_request_rx) =
            mpsc::channel::<WithdrawRequest>(100);

        let database = self.database.to_owned();
        let provider = self.provider.to_owned();
        let contract_events = self.contract.events();
        let join_handle: tokio::task::JoinHandle<Result<(), BlockchainManagerError>> = tokio::spawn(
            async move {
                let mut blocks_stream = provider.subscribe_blocks().await?;
                let mut events_stream = contract_events.stream_with_meta().await?;

                loop {
                    select! {
                        _ = &mut shutdown_rx => {
                            break;
                        },
                        Some(request) = insert_withdraw_request_rx.recv() => {
                            if let Err(err) = async {
                                let mut t = database.begin().await.map_err(|e| Status::aborted(e.to_string()))?;
                                t.commit().await?;
                                Ok::<(), BlockchainManagerError>(())
                            }.await {
                                tracing::error!("{err}");
                            }
                        },
                        Some(Ok((filter, meta))) = events_stream.next() => {
                            if let Err(err) = async {
                                let mut t = database.begin().await.map_err(|e| Status::aborted(e.to_string()))?;
                                match filter {
                                    TreasuryEvents::WithdrawFilter(filter) => {
                                        let event = WithdrawEvent::from_filter(&mut t,   filter).await?;
                                    }
                                    _ => (),
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
            insert_withdraw_request_tx,
        }
    }
}

#[derive(Debug)]
pub struct WithdrawsBlockchainManagerController {
    shutdown_tx: oneshot::Sender<()>,
    insert_withdraw_request_tx: mpsc::Sender<WithdrawRequest>,
    join_handle: tokio::task::JoinHandle<Result<(), BlockchainManagerError>>,
}
impl WithdrawsBlockchainManagerController {
    pub async fn shutdown(self) -> Result<Result<(), BlockchainManagerError>, JoinError> {
        if self.shutdown_tx.send(()).is_err() {
            tracing::error!("Error: shutdown");
        }
        Ok(self.join_handle.await?)
    }

    pub async fn withdraw(
        &self,
        request: WithdrawRequest,
    ) -> Result<(), mpsc::error::SendError<WithdrawRequest>> {
        self.insert_withdraw_request_tx.send(request).await
    }
}
