pub mod models;

use ethers::{
    prelude::LogMeta,
    providers::{Middleware, Provider, Ws},
};
use fraction::Fraction;
use futures::stream::StreamExt;
use num_bigint::BigInt;
use sqlx::PgPool;
use tokio::{select, sync::oneshot, task::JoinError};
use tonic::{transport::Channel, Status};

use crate::{
    contracts::treasury::{DepositFilter, Treasury, TreasuryEvents},
    database::{
        managers::deposits::DepositsManager,
        projections::deposit::{Deposit, DepositInsert},
    },
    engine_base::{self, engine_client::EngineClient},
    models::BlockchainManagerError,
};

use self::models::DepositQueue;

pub struct DepositsBlockchainManager {
    pub database: PgPool,
    pub provider: Provider<Ws>,
    pub contract: Treasury<Provider<Ws>>,
    pub confirmations: BigInt,
}

impl DepositsBlockchainManager {
    pub async fn start(
        &self,
        mut engine_client: EngineClient<Channel>,
    ) -> DepositsBlockchainManagerController {
        let (shutdown_tx, mut shutdown_rx) = oneshot::channel::<()>();

        let database = self.database.to_owned();
        let provider = self.provider.to_owned();
        let contract = self.contract.to_owned();
        let confirmations = Fraction::from_raw((BigInt::from(0), self.confirmations.to_owned()))
            .unwrap_or_default();

        let join_handle: tokio::task::JoinHandle<Result<(), BlockchainManagerError>> = tokio::spawn(
            async move {
                let mut blocks_stream = provider.subscribe_blocks().await?;
                let mut deposits_queue = DepositQueue::<Deposit>::new(&provider);

                loop {
                    select! {
                        _ = &mut shutdown_rx => {
                            break;
                        },
                        Some(block) = blocks_stream.next() => {
                            let events = contract.events().at_block_hash(block.hash.unwrap_or_default()).query_with_meta().await?.into_iter()
                            .filter_map(|(ev, log)| match ev {
                                TreasuryEvents::DepositFilter(ev) => Some((ev, log)),
                                _ => None
                            }).collect::<Vec<(DepositFilter, LogMeta)>>();

                            if let Err(err) = async {
                                let mut t = database.begin().await.map_err(|e| Status::aborted(e.to_string()))?;

                                for (event, meta) in events {
                                    let insert = DepositInsert::from_filter(&mut t, &event, &meta, confirmations.to_owned()).await?;
                                    let deposit = DepositsManager::insert(&mut t, &insert).await?;
                                    deposits_queue.insert(deposit, meta.transaction_hash).await?;
                                }

                                let (confirmed_deposit, not_confirmed_deposits) = deposits_queue.eval(&block).await;
                                for deposit in not_confirmed_deposits.into_iter() {
                                    DepositsManager::update(&mut t, deposit).await?;
                                }
                                for deposit in confirmed_deposit.iter().cloned() {
                                    DepositsManager::update(&mut t, deposit).await?;
                                }

                                let mut transfers: Vec<engine_base::TransferRequest> = Vec::new();

                                for deposit in confirmed_deposit {
                                    transfers.push(deposit.as_transfer_request(&mut t).await?);
                                }

                                t.commit().await?;
                                tracing::info!("commited");

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
        }
    }
}

#[derive(Debug)]
pub struct DepositsBlockchainManagerController {
    shutdown_tx: oneshot::Sender<()>,
    join_handle: tokio::task::JoinHandle<Result<(), BlockchainManagerError>>,
}
impl DepositsBlockchainManagerController {
    pub async fn shutdown(self) -> Result<Result<(), BlockchainManagerError>, JoinError> {
        if self.shutdown_tx.send(()).is_err() {
            tracing::error!("Error: shutdown");
        }
        self.join_handle.await
    }
}
