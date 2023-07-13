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

use crate::{
    contracts::{block_distance, current_block, treasury::Treasury},
    database::managers::{notification::NotificationManagerOutput, valuts::ValutsManager},
    models::BlockchainManagerError,
    submission::SubmissionQueue,
};

pub struct ValutsBlockchainManager {
    pub database: PgPool,
    pub provider: Provider<Ws>,
    pub contract: Treasury<SignerMiddleware<Provider<Ws>, Wallet<SigningKey>>>,
}

impl ValutsBlockchainManager {
    pub async fn start(
        &self,
        mut notifications: mpsc::Receiver<NotificationManagerOutput>,
    ) -> ValutsBlockchainManagerController {
        let (shutdown_tx, mut shutdown_rx) = oneshot::channel::<()>();
        let (set_changes_threshold_tx, mut set_changes_threshold_rx) = mpsc::channel::<usize>(100);
        let (set_blocks_threshold_tx, mut set_blocks_threshold_rx) = mpsc::channel::<usize>(100);
        let (push_tx, mut push_rx) = mpsc::channel::<()>(100);

        let contract = self.contract.to_owned();
        let valuts_manager = ValutsManager::new(self.database.to_owned());
        let provider = self.provider.to_owned();
        let join_handle: tokio::task::JoinHandle<Result<(), BlockchainManagerError>> =
            tokio::spawn(async move {
                let mut submission_queue = SubmissionQueue::new(&valuts_manager, &contract);
                let mut changes_threshold = 10;
                let mut blocks_threshold = 10;
                let mut last_block = current_block(&provider).await?;
                let mut blocks_stream = provider.subscribe_blocks().await?;

                loop {
                    select! {
                        _ = &mut shutdown_rx => {
                            break;
                        },
                        Some(threshold) = set_changes_threshold_rx.recv() => {
                            changes_threshold = threshold;
                        }
                        Some(threshold) = set_blocks_threshold_rx.recv() => {
                            blocks_threshold = threshold;
                        }
                        Some(_) = push_rx.recv() => {
                            if let Err(err) = async {
                                submission_queue.submit().await?;
                                Ok::<(), BlockchainManagerError>(())
                            }.await {
                                tracing::error!("{err}")
                            }
                        }
                        Some(NotificationManagerOutput::Valuts(valuts)) = notifications.recv() => {
                            if let Err(err) = async {
                                for id in valuts.into_iter().map(|f| f.id) {
                                    submission_queue.enqueue(id);
                                }
                                if submission_queue.size() >= changes_threshold{
                                    submission_queue.submit().await?;
                                }
                                Ok::<(), BlockchainManagerError>(())
                            }.await {
                                tracing::error!("{err}")
                            }
                        }
                        Some(block) = blocks_stream.next() => {
                            if let Err(err) = async {
                                if block_distance(&last_block, &block).await? >= blocks_threshold {
                                    if submission_queue.size() > 0_usize {
                                        submission_queue.submit().await?;
                                    }
                                    last_block = block;
                                }
                                Ok::<(), BlockchainManagerError>(())
                            }.await {
                                tracing::error!("{err}");
                            }
                        }
                    }
                }
                Ok(())
            });
        ValutsBlockchainManagerController {
            shutdown_tx,
            join_handle,
            set_changes_threshold_tx,
            set_blocks_threshold_tx,
            push_tx,
        }
    }
}

#[derive(Debug)]
pub struct ValutsBlockchainManagerController {
    shutdown_tx: oneshot::Sender<()>,
    join_handle: tokio::task::JoinHandle<Result<(), BlockchainManagerError>>,
    set_changes_threshold_tx: mpsc::Sender<usize>,
    set_blocks_threshold_tx: mpsc::Sender<usize>,
    push_tx: mpsc::Sender<()>,
}
impl ValutsBlockchainManagerController {
    pub async fn shutdown(self) -> Result<Result<(), BlockchainManagerError>, JoinError> {
        if self.shutdown_tx.send(()).is_err() {
            tracing::error!("Error: shutdown");
        }
        Ok(self.join_handle.await?)
    }

    pub async fn set_changes_threshold(
        &self,
        threshold: usize,
    ) -> Result<(), mpsc::error::SendError<usize>> {
        self.set_changes_threshold_tx.send(threshold).await
    }

    pub async fn set_blocks_threshold(
        &self,
        threshold: usize,
    ) -> Result<(), mpsc::error::SendError<usize>> {
        self.set_blocks_threshold_tx.send(threshold).await
    }

    pub async fn push(&self) -> Result<(), mpsc::error::SendError<()>> {
        self.push_tx.send(()).await
    }
}
