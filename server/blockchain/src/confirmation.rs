use ethereum_types::H256;
use ethers::{
    providers::{Provider, ProviderError, Ws},
    types::Block,
};
use futures::{
    stream::{self, StreamExt},
    TryFutureExt,
};

use crate::{
    contracts::{confirmations, transaction_block},
    database::projections::Confirmable,
};
#[derive(Clone)]
pub struct ConfirmationQueueEntry<E: Confirmable> {
    entity: E,
    tx_hash: H256,
    tx_block: Block<H256>,
}

impl<E: Confirmable> ConfirmationQueueEntry<E> {
    pub fn new(entity: E, tx_hash: H256, tx_block: Block<H256>) -> Self {
        Self {
            entity,
            tx_hash,
            tx_block,
        }
    }
}

pub struct ConfirmationQueue<'a, E: Confirmable> {
    provider: &'a Provider<Ws>,
    entries: Vec<ConfirmationQueueEntry<E>>,
}

impl<'a, E: Confirmable> ConfirmationQueue<'a, E> {
    pub fn new(provider: &'a Provider<Ws>) -> Self {
        Self {
            provider,
            entries: Vec::new(),
        }
    }

    pub async fn insert(&mut self, entity: E, tx_hash: H256) -> Result<(), ProviderError> {
        Ok(self.entries.push(ConfirmationQueueEntry::new(
            entity,
            tx_hash.clone(),
            transaction_block(&self.provider, tx_hash).await?,
        )))
    }

    async fn eval_ready(
        entries: Vec<ConfirmationQueueEntry<E>>,
        block: &Block<H256>,
    ) -> (
        Vec<ConfirmationQueueEntry<E>>,
        Vec<ConfirmationQueueEntry<E>>,
    ) {
        stream::iter(entries.into_iter())
            .map(|f| (f, block))
            .fold(
                (vec![], vec![]),
                |(mut ready, mut not_ready), (mut f, block)| async move {
                    if let Ok(confirmations) = confirmations(block, &f.tx_block).await {
                        f.entity.set(confirmations);
                        if f.entity.is_confirmed() {
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
        entries: Vec<ConfirmationQueueEntry<E>>,
        provider: &Provider<Ws>,
        block: &Block<H256>,
    ) -> (
        Vec<ConfirmationQueueEntry<E>>,
        Vec<ConfirmationQueueEntry<E>>,
    ) {
        stream::iter(entries.into_iter())
            .map(|f| (f, provider, block))
            .fold(
                (vec![], vec![]),
                |(mut confirmed, mut not_confirmed), (mut f, provider, block)| async move {
                    if let Ok(confirmations) = transaction_block(provider, f.tx_hash)
                        .and_then(|tx_block| async move { confirmations(block, &tx_block).await })
                        .await
                    {
                        f.entity.set(confirmations);
                        if f.entity.is_confirmed() {
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

    pub async fn confirmation_step(&mut self, block: &Block<H256>) -> sqlx::Result<Vec<E>> {
        let (ready, mut not_ready) =
            Self::eval_ready(self.entries.drain(0..).collect(), &block).await;
        let (confirmed, not_confirmed) = Self::eval_confirmed(ready, &self.provider, &block).await;
        not_ready.extend(not_confirmed.into_iter());
        self.entries.extend(not_ready);

        Ok(confirmed.into_iter().map(|f| f.entity).collect())
    }
}
