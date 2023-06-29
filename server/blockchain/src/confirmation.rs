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
use num_bigint::BigInt;
use num_traits::Zero;
use sqlx::PgPool;

use crate::{
    contracts::{confirmations, transaction_block},
    database::managers::FlowManager,
};
#[derive(Clone)]
pub struct ConfirmationQueueEntry<E> {
    entity: E,
    tx_hash: H256,
    tx_block: Block<H256>,
    confirmations: Fraction,
}

impl<E> ConfirmationQueueEntry<E> {
    pub fn new(entity: E, tx_hash: H256, tx_block: Block<H256>) -> Self {
        Self {
            entity,
            tx_hash,
            tx_block,
            confirmations: Fraction::from((BigInt::zero(), BigInt::from(1000))),
        }
    }
}

pub struct ConfirmationQueue<'a, Manager, E>
where
    Manager: FlowManager,
{
    database: &'a PgPool,
    manager: &'a Manager,
    provider: &'a Provider<Ws>,
    entries: Vec<ConfirmationQueueEntry<E>>,
}

impl<'a, Manager, E> ConfirmationQueue<'a, Manager, E>
where
    Manager: FlowManager,
{
    pub fn new(database: &'a PgPool, manager: &'a Manager, provider: &'a Provider<Ws>) -> Self {
        Self {
            database,
            manager,
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
                    if let Ok(Some(confirmations)) = confirmations(block, &f.tx_block)
                        .await
                        .map(|e| Fraction::from_raw((e.into(), f.confirmations.denom().clone())))
                    {
                        match confirmations.cmp(&Fraction::one()) {
                            Ordering::Equal | Ordering::Greater => {
                                f.confirmations = confirmations;
                                ready.push(f);
                            }
                            Ordering::Less => {
                                f.confirmations = confirmations;
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
                    if let Ok(Some(confirmations)) = transaction_block(provider, f.tx_hash)
                        .and_then(|tx_block| async move { confirmations(block, &tx_block).await })
                        .await
                        .map(|e| Fraction::from_raw((e.into(), f.confirmations.denom().clone())))
                    {
                        match confirmations.cmp(&Fraction::one()) {
                            Ordering::Equal | Ordering::Greater => {
                                f.confirmations = confirmations;
                                confirmed.push(f);
                            }
                            Ordering::Less => {
                                f.confirmations = confirmations;
                                not_confirmed.push(f);
                            }
                        }
                    }
                    (confirmed, not_confirmed)
                },
            )
            .await
    }

    pub async fn confirmation_step(&mut self, block: Block<H256>) -> sqlx::Result<Vec<E>> {
        let (ready, mut not_ready) =
            Self::eval_ready(self.entries.drain(0..).collect(), &block).await;
        let (confirmed, not_confirmed) = Self::eval_confirmed(ready, &self.provider, &block).await;
        not_ready.extend(not_confirmed.into_iter());
        self.entries.extend(not_ready);

        Ok(confirmed.into_iter().map(|f| f.entity).collect())
    }
}
