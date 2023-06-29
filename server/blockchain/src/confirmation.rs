use std::cmp::Ordering;

use ethereum_types::H256;
use ethers::{
    providers::{Provider, ProviderError, Ws},
    types::Block,
};
use fraction::{num_traits::One, Fraction};
use futures::{
    stream::{self, StreamExt},
    TryFutureExt,
};
use sqlx::PgPool;

use crate::{
    contracts::{confirmations, transaction_block},
    database::{managers::FlowManager, projections::Flow},
};

#[derive(Clone)]
pub struct ConfirmationQueueEntry {
    flow: Flow,
    tx_block: Block<H256>,
}

pub struct ConfirmationQueue<T>
where
    T: FlowManager,
{
    database: PgPool,
    manager: T,
    provider: Provider<Ws>,
    entries: Vec<ConfirmationQueueEntry>,
}

impl<T> ConfirmationQueue<T>
where
    T: FlowManager,
{
    pub fn new(database: PgPool, manager: T, provider: Provider<Ws>) -> Self {
        Self {
            database,
            manager,
            provider,
            entries: Vec::new(),
        }
    }

    pub async fn insert(&mut self, flow: Flow) -> Result<(), ProviderError> {
        Ok(self.entries.push(ConfirmationQueueEntry {
            tx_block: transaction_block(&self.provider, *flow.tx_hash).await?,
            flow,
        }))
    }

    async fn eval_ready(
        entries: Vec<ConfirmationQueueEntry>,
        block: &Block<H256>,
    ) -> (Vec<ConfirmationQueueEntry>, Vec<ConfirmationQueueEntry>) {
        stream::iter(entries.into_iter())
            .map(|f| (f, block))
            .fold(
                (vec![], vec![]),
                |(mut ready, mut not_ready), (mut f, block)| async move {
                    if let Ok(Some(confirmations)) =
                        confirmations(block, &f.tx_block).await.map(|e| {
                            Fraction::from_raw((e.into(), f.flow.confirmations.denom().clone()))
                        })
                    {
                        match confirmations.cmp(&Fraction::one()) {
                            Ordering::Equal | Ordering::Greater => {
                                f.flow.confirmations = confirmations;
                                ready.push(f);
                            }
                            Ordering::Less => {
                                f.flow.confirmations = confirmations;
                                not_ready.push(f);
                            }
                        }
                    }
                    (ready, not_ready)
                },
            )
            .await
    }

    async fn eval_confirmed(
        entries: Vec<ConfirmationQueueEntry>,
        provider: &Provider<Ws>,
        block: &Block<H256>,
    ) -> (Vec<ConfirmationQueueEntry>, Vec<ConfirmationQueueEntry>) {
        stream::iter(entries.into_iter())
            .map(|f| (f, provider, block))
            .fold(
                (vec![], vec![]),
                |(mut confirmed, mut not_confirmed), (mut f, provider, block)| async move {
                    if let Ok(Some(confirmations)) = transaction_block(provider, *f.flow.tx_hash)
                        .and_then(|tx_block| async move { confirmations(block, &tx_block).await })
                        .await
                        .map(|e| {
                            Fraction::from_raw((e.into(), f.flow.confirmations.denom().clone()))
                        })
                    {
                        match confirmations.cmp(&Fraction::one()) {
                            Ordering::Equal | Ordering::Greater => {
                                f.flow.confirmations = confirmations;
                                confirmed.push(f);
                            }
                            Ordering::Less => {
                                f.flow.confirmations = confirmations;
                                not_confirmed.push(f);
                            }
                        }
                    }
                    (confirmed, not_confirmed)
                },
            )
            .await
    }

    pub async fn confirmation_step(&mut self, block: Block<H256>) -> sqlx::Result<Vec<Flow>> {
        let mut t = self.database.begin().await?;

        let f: sqlx::Result<(Vec<_>, Vec<_>)> = async {
            let (ready, mut not_ready) = Self::eval_ready(self.entries.to_owned(), &block).await;
            let (confirmed, not_confirmed) =
                Self::eval_confirmed(ready, &self.provider, &block).await;
            not_ready.extend(not_confirmed.into_iter());

            // update in db
            for entry in not_ready.clone().into_iter().map(|f| f.flow) {
                self.manager.update(&mut t, entry).await?;
            }

            for entry in confirmed.clone().into_iter().map(|f| f.flow) {
                self.manager.update(&mut t, entry).await?;
            }

            Ok((confirmed, not_ready))
        }
        .await;

        match f {
            Ok((confirmed, not_ready)) => {
                t.commit().await?;
                self.entries = not_ready;
                Ok(confirmed.into_iter().map(|f| f.flow).collect())
            }
            Err(e) => {
                t.rollback().await?;
                Err(e)
            }
        }
    }
}
