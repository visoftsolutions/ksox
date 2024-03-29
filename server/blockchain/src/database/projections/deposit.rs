use chrono::{DateTime, Utc};
use ethers::prelude::LogMeta;
use evm::{address::Address, confirmations::Confirmations, txhash::TxHash};
use fraction::Fraction;
use num_bigint::BigInt;
use num_traits::Zero;
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

use super::Confirmable;
use crate::{
    blockchain_engine::models::BlockchainEngineError,
    contracts::treasury::DepositFilter,
    database::managers::{assets::AssetsManager, users::UsersManager, valuts::ValutsManager},
    engine_base,
};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Deposit {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub last_modification_at: DateTime<Utc>,
    pub owner: Address,
    pub spender: Address,
    pub asset: Address,
    pub amount: Fraction,
    pub tx_hash: TxHash,
    pub confirmations: Confirmations,
}

impl Deposit {
    pub async fn as_transfer_request<'t>(
        &self,
        transaction: &'t mut Transaction<'_, Postgres>,
    ) -> Result<engine_base::TransferRequest, BlockchainEngineError> {
        let taker_id = UsersManager::get_or_create_by_address(transaction, &self.owner)
            .await?
            .id;
        let asset_id = AssetsManager::get_by_address(transaction, &self.asset)
            .await?
            .id;
        let from_valut_id = ValutsManager::get_or_create(transaction, Uuid::default(), asset_id)
            .await?
            .id;
        let to_valut_id = ValutsManager::get_or_create(transaction, taker_id, asset_id)
            .await?
            .id;
        Ok::<engine_base::TransferRequest, BlockchainEngineError>(engine_base::TransferRequest {
            from_valut_id: from_valut_id.to_string(),
            to_valut_id: to_valut_id.to_string(),
            asset_id: asset_id.to_string(),
            amount: serde_json::to_string(&self.amount)?,
            fee_fraction: serde_json::to_string(&Fraction::zero())?,
        })
    }
}

impl Confirmable for Deposit {
    fn set(&mut self, actual: BigInt) {
        self.confirmations = Confirmations::from_raw(actual, self.confirmations.desired().to_owned());
    }
    fn is_confirmed(&self) -> bool {
        self.confirmations.actual() >= self.confirmations.desired()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DepositInsert {
    pub owner: Address,
    pub spender: Address,
    pub asset: Address,
    pub amount: Fraction,
    pub tx_hash: TxHash,
    pub confirmations: Confirmations,
}

impl DepositInsert {
    pub async fn from_filter<'t>(
        t: &'t mut Transaction<'_, Postgres>,
        filter: &DepositFilter,
        meta: &LogMeta,
        confirmations: Confirmations,
    ) -> sqlx::Result<Self> {
        let asset = AssetsManager::get_by_address(t, &Address(filter.token)).await?;
        Ok(Self {
            owner: filter.owner.into(),
            spender: filter.spender.into(),
            asset: filter.token.into(),
            tx_hash: meta.transaction_hash.into(),
            amount: Fraction::from(filter.amount) / asset.decimals,
            confirmations,
        })
    }
}
