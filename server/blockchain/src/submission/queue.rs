use std::collections::HashSet;

use ethereum_types::U256;
use ethers::{
    prelude::ContractError,
    providers::{Provider, Ws},
};
use thiserror::Error;
use uuid::Uuid;

use crate::{
    contracts::treasury::{BalanceUpdate, Treasury},
    database::managers::valuts::ValutsManager,
};

pub struct SubmissionQueue<'a> {
    entries: HashSet<Uuid>,
    valuts_manager: &'a ValutsManager,
    contract: &'a Treasury<Provider<Ws>>,
}

impl<'a> SubmissionQueue<'a> {
    pub fn new(valuts_manager: &'a ValutsManager, contract: &'a Treasury<Provider<Ws>>) -> Self {
        Self {
            entries: HashSet::new(),
            valuts_manager,
            contract,
        }
    }

    pub async fn submit(&mut self) -> Result<(), SubmissionQueueError> {
        let updates: Vec<BalanceUpdate> = self
            .valuts_manager
            .get_by_id(self.entries.iter().cloned().collect())
            .await?
            .into_iter()
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

        match self.contract.set_balances(updates).await {
            Ok(()) => {
                self.entries.clear();
                Ok(())
            }
            Err(err) => Err(err.into()),
        }
    }

    pub fn enqueue(&mut self, id: Uuid) -> bool {
        self.entries.insert(id)
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
    ContractError(#[from] ContractError<Provider<Ws>>),
}
