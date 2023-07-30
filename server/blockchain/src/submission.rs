use std::collections::HashSet;

use ethereum_types::U256;
use ethers::{
    prelude::{k256::ecdsa::SigningKey, ContractError, SignerMiddleware},
    providers::{Provider, Ws},
    signers::Wallet,
};
use thiserror::Error;

use crate::{
    contracts::treasury::{BalanceUpdate, Treasury},
    database::{managers::valuts::ValutsManager, projections::valut::ValutFinite},
};

pub struct SubmissionQueue<'a> {
    entries: HashSet<ValutFinite>,
    contract: &'a Treasury<SignerMiddleware<Provider<Ws>, Wallet<SigningKey>>>,
}

impl<'a> SubmissionQueue<'a> {
    pub fn new(contract: &'a Treasury<SignerMiddleware<Provider<Ws>, Wallet<SigningKey>>>) -> Self {
        Self {
            entries: HashSet::new(),
            contract,
        }
    }

    pub async fn submit(&mut self) -> Result<(), SubmissionQueueError> {
        let updates: Vec<BalanceUpdate> = self
            .entries
            .drain()
            .map(|f| BalanceUpdate {
                token_address: *f.asset_address,
                user_address: *f.user_address,
                value: U256::from_little_endian(
                    (*(f.balance * f.decimals))
                        .to_integer()
                        .to_bytes_le()
                        .1
                        .as_slice(),
                ),
            })
            .collect();
        
        tracing::info!("{:?}", updates);
        match self.contract.set_balances(updates).send().await {
            Ok(a) => {
                tracing::info!("{:?}", a);
                self.entries.clear();
                Ok(())
            }
            Err(err) => Err(err.into()),
        }
    }

    pub fn enqueue(&mut self, valut: ValutFinite) -> bool {
        self.entries.insert(valut)
    }

    pub fn size(&self) -> usize {
        self.entries.len()
    }
}

#[derive(Error, Debug)]
pub enum SubmissionQueueError {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    #[error(transparent)]
    ContractError(#[from] ContractError<SignerMiddleware<Provider<Ws>, Wallet<SigningKey>>>),
}
