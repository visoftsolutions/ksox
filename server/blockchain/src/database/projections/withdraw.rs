use chrono::{DateTime, NaiveDateTime, Utc};
use evm::address::Address;
use fraction::Fraction;
use num_traits::ToPrimitive;
use num_traits::Zero;
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

use crate::{
    blockchain_engine::models::BlockchainEngineError,
    contracts::treasury::WithdrawFilter,
    database::managers::{assets::AssetsManager, users::UsersManager, valuts::ValutsManager},
    engine_base,
};

use super::Expirable;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Withdraw {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub last_modification_at: DateTime<Utc>,
    pub owner: Address,
    pub spender: Address,
    pub asset: Address,
    pub amount: Fraction,
    pub nonce: Fraction,
    pub deadline: DateTime<Utc>,
}

impl Withdraw {
    pub async fn as_transfer_request<'t>(
        &self,
        transaction: &'t mut Transaction<'_, Postgres>,
    ) -> Result<engine_base::TransferRequest, BlockchainEngineError> {
        let maker_id = UsersManager::get_by_address(transaction, &self.spender)
            .await?
            .id;
        let asset_id = AssetsManager::get_by_address(transaction, &self.asset)
            .await?
            .id;
        let from_valut_id = ValutsManager::get_or_create(transaction, maker_id, asset_id)
            .await?
            .id;
        let to_valut_id = ValutsManager::get_or_create(transaction, Uuid::default(), asset_id)
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

impl Expirable for Withdraw {
    fn is_expired(&self, time: &DateTime<Utc>) -> bool {
        time > &self.deadline
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct WithdrawInsert {
    pub owner: Address,
    pub spender: Address,
    pub asset: Address,
    pub amount: Fraction,
    pub nonce: Fraction,
    pub deadline: DateTime<Utc>,
}

impl WithdrawInsert {
    pub async fn from_filter<'t>(
        t: &'t mut Transaction<'_, Postgres>,
        filter: &WithdrawFilter,
    ) -> sqlx::Result<Self> {
        let asset = AssetsManager::get_by_address(t, &Address(filter.token)).await?;
        let naive_datetime = NaiveDateTime::from_timestamp_millis(
            (Fraction::from(filter.deadline) * Fraction::from(1000))
                .to_integer()
                .to_i64()
                .unwrap_or_default(),
        )
        .unwrap_or_default();
        Ok(Self {
            owner: filter.owner.into(),
            spender: filter.spender.into(),
            asset: filter.token.into(),
            amount: Fraction::from(filter.amount) / asset.decimals,
            nonce: Fraction::from(filter.nonce),
            deadline: DateTime::<Utc>::from_utc(naive_datetime, Utc),
        })
    }
}
