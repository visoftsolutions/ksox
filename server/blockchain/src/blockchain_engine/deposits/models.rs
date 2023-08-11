use ethereum_types::H256;
use ethers::{
    providers::{Provider, ProviderError, Ws},
    types::Block,
};
use futures::{
    stream::{self, StreamExt},
    TryFutureExt,
};
use thiserror::Error;

use crate::{
    contracts::{block_distance, transaction_block},
    database::projections::Confirmable,
};
#[derive(Clone)]
pub struct DepositQueueEntry<E: Confirmable + Clone> {
    entry: E,
    tx_hash: H256,
    tx_block: Block<H256>,
}

impl<E: Confirmable + Clone> DepositQueueEntry<E> {
    pub fn new(entry: E, tx_hash: H256, tx_block: Block<H256>) -> Self {
        Self {
            entry,
            tx_hash,
            tx_block,
        }
    }
}

pub struct DepositQueue<'a, E: Confirmable + Clone> {
    provider: &'a Provider<Ws>,
    entries: Vec<DepositQueueEntry<E>>,
}

impl<'a, E: Confirmable + Clone> DepositQueue<'a, E> {
    pub fn new(provider: &'a Provider<Ws>) -> Self {
        Self {
            provider,
            entries: Vec::new(),
        }
    }

    pub async fn insert(&mut self, entity: E, tx_hash: H256) -> Result<(), DepositQueueError> {
        self.entries.push(DepositQueueEntry::new(
            entity,
            tx_hash,
            transaction_block(self.provider, tx_hash).await?,
        ));
        Ok(())
    }

    async fn eval_ready(
        entries: Vec<DepositQueueEntry<E>>,
        block: &Block<H256>,
    ) -> (Vec<DepositQueueEntry<E>>, Vec<DepositQueueEntry<E>>) {
        stream::iter(entries.into_iter())
            .map(|f| (f, block))
            .fold(
                (vec![], vec![]),
                |(mut ready, mut not_ready), (mut f, block)| async move {
                    if let Ok(confirmations) = block_distance(block, &f.tx_block).await {
                        f.entry.set(confirmations);
                        if f.entry.is_confirmed() {
                            ready.push(f);
                        } else {
                            not_ready.push(f);
                        }
                    }
                    (ready, not_ready)
                },
            )
            .await
    }

    async fn eval_confirmed(
        entries: Vec<DepositQueueEntry<E>>,
        provider: &Provider<Ws>,
        block: &Block<H256>,
    ) -> (Vec<DepositQueueEntry<E>>, Vec<DepositQueueEntry<E>>) {
        stream::iter(entries.into_iter())
            .map(|f| (f, provider, block))
            .fold(
                (vec![], vec![]),
                |(mut confirmed, mut not_confirmed), (mut f, provider, block)| async move {
                    if let Ok(confirmations) = transaction_block(provider, f.tx_hash)
                        .and_then(|tx_block| async move { block_distance(block, &tx_block).await })
                        .await
                    {
                        f.entry.set(confirmations);
                        if f.entry.is_confirmed() {
                            confirmed.push(f);
                        } else {
                            not_confirmed.push(f);
                        }
                    }
                    (confirmed, not_confirmed)
                },
            )
            .await
    }

    pub async fn eval(&mut self, block: &Block<H256>) -> (Vec<E>, Vec<E>) {
        let (ready, mut not_ready) =
            Self::eval_ready(self.entries.drain(0..).collect(), block).await;
        let (confirmed, not_confirmed) = Self::eval_confirmed(ready, self.provider, block).await;
        not_ready.extend(not_confirmed.into_iter());
        self.entries.extend(not_ready.clone());

        (
            confirmed.into_iter().map(|f| f.entry).collect(),
            not_ready.into_iter().map(|f| f.entry).collect(),
        )
    }
}

#[derive(Error, Debug)]
pub enum DepositQueueError {
    #[error(transparent)]
    ProviderError(#[from] ProviderError),
}
