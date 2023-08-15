pub mod models;

use std::str::FromStr;

use chrono::{DateTime, NaiveDateTime, Utc};
use ethereum_types::U256;
use ethers::{
    prelude::LogMeta,
    providers::{Middleware, Provider, Ws},
    signers::{LocalWallet, Signer},
    types::Signature,
};

use sqlx::PgPool;
use tokio::{
    select,
    sync::{mpsc, oneshot},
    task::JoinError,
};
use tokio_stream::StreamExt;
use tonic::{transport::Channel, Status};
use uuid::Uuid;

use crate::{
    blockchain_engine::withdraws::models::WithdrawQueueValue,
    contracts::{
        treasury::{Treasury, TreasuryEvents, WithdrawFilter},
        Permit,
    },
    database::{
        managers::{assets::AssetsManager, withdraws::WithdrawsManager},
        projections::withdraw::WithdrawInsert,
    },
    engine_base::{self, engine_client::EngineClient},
    models::BlockchainManagerError,
};

use self::models::WithdrawQueue;

use super::models::WithdrawPermitRequest;

pub struct WithdrawsBlockchainManager {
    pub database: PgPool,
    pub provider: Provider<Ws>,
    pub contract: Treasury<Provider<Ws>>,
    pub contract_key_wallet: LocalWallet,
}

impl WithdrawsBlockchainManager {
    pub async fn start(
        &self,
        mut engine_client: EngineClient<Channel>,
    ) -> WithdrawsBlockchainManagerController {
        let (shutdown_tx, mut shutdown_rx) = oneshot::channel::<()>();
        let (insert_withdraw_event_tx, mut insert_withdraw_event_rx) =
            mpsc::channel::<(WithdrawInsert, WithdrawQueueValue)>(100);

        let database = self.database.to_owned();
        let provider = self.provider.to_owned();
        let contract = self.contract.to_owned();

        let join_handle: tokio::task::JoinHandle<Result<(), BlockchainManagerError>> = tokio::spawn(
            async move {
                let mut blocks_stream = provider.subscribe_blocks().await?;
                let mut withdraw_queue = WithdrawQueue::new();

                loop {
                    select! {
                        _ = &mut shutdown_rx => {
                            break;
                        },
                        Some((key, value)) = insert_withdraw_event_rx.recv() => {
                            withdraw_queue.insert(key, value);
                        },
                        Some(block) = blocks_stream.next() => {
                            let time = block.time()?;
                            let events = contract.events().at_block_hash(block.hash.unwrap_or_default()).query_with_meta().await?.into_iter()
                            .filter_map(|(ev, log)| match ev {
                                TreasuryEvents::WithdrawFilter(ev) => Some((ev, log)),
                                _ => None
                            }).collect::<Vec<(WithdrawFilter, LogMeta)>>();

                            if let Err(err) = async {
                                let mut t = database.begin().await.map_err(|e| Status::aborted(e.to_string()))?;

                                for (event, _) in events {
                                    let key = WithdrawInsert::from_filter(&mut t, &event).await?;
                                    withdraw_queue.remove(&key);
                                }

                                for expired in withdraw_queue.eval(&time) {
                                    engine_client.revert_transfer(engine_base::RevertTransferRequest {
                                        transfer_id: expired.transfer.to_string()
                                    }).await?;
                                }

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
            database: self.database.to_owned(),
            contract: self.contract.to_owned(),
            contract_key_wallet: self.contract_key_wallet.to_owned(),
        }
    }
}

#[derive(Debug)]
pub struct WithdrawsBlockchainManagerController {
    shutdown_tx: oneshot::Sender<()>,
    insert_withdraw_event_tx: mpsc::Sender<(WithdrawInsert, WithdrawQueueValue)>,
    join_handle: tokio::task::JoinHandle<Result<(), BlockchainManagerError>>,
    database: PgPool,
    contract: Treasury<Provider<Ws>>,
    contract_key_wallet: LocalWallet,
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
        request: WithdrawPermitRequest,
        mut engine_client: EngineClient<Channel>,
    ) -> Result<Signature, BlockchainManagerError> {
        let mut t = self.database.begin().await?;

        let timestamp = request.deadline.timestamp();
        let deadline = DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp_millis(timestamp * 1000).unwrap_or_default(),
            Utc,
        );

        let insert = WithdrawInsert {
            owner: self.contract_key_wallet.address().into(),
            spender: request.spender.to_owned(),
            asset: request.asset.to_owned(),
            amount: request.amount.to_owned(),
            nonce: self.contract.nonces(*request.spender).await?.into(),
            deadline: deadline,
        };

        let withdraw = WithdrawsManager::insert(&mut t, &insert).await?;

        let transfer_id = Uuid::from_str(
            engine_client
                .transfer(withdraw.as_transfer_request(&mut t).await?)
                .await?
                .into_inner()
                .transfer_id
                .as_str(),
        )?;

        let asset = AssetsManager::get_by_address(&mut t, &withdraw.asset).await?;

        t.commit().await?;

        self.insert_withdraw_event_tx
            .send((
                insert,
                WithdrawQueueValue {
                    withdraw: withdraw.to_owned(),
                    transfer: transfer_id,
                },
            ))
            .await?;

        let permit = Permit {
            owner: withdraw.owner.into(),
            spender: withdraw.spender.into(),
            token: withdraw.asset.into(),
            value: (withdraw.amount * asset.decimals).into(),
            nonce: withdraw.nonce.into(),
            deadline: U256::from(timestamp),
        };

        Ok(self.contract_key_wallet.sign_typed_data(&permit).await?)
    }
}
