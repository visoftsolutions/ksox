use chrono::{DateTime, Utc};
use evm::address::Address;
use fraction::Fraction;
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

use crate::{
    blockchain_engine::models::BlockchainEngineError,
    database::managers::{assets::AssetsManager, users::UsersManager},
    engine_base,
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Withdraw {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub last_modification_at: DateTime<Utc>,
    pub owner: Address,
    pub spender: Address,
    pub asset: Address,
    pub amount: Fraction,
    pub nonce: i64,
    pub deadline: DateTime<Utc>,
}

pub async fn withdraw_to_transfer_request(
    pool: &mut Transaction<'_, Postgres>,
    withdraw: Withdraw,
) -> Result<engine_base::TransferRequest, BlockchainEngineError> {
    let maker_id = UsersManager::get_by_address(pool, withdraw.spender)
        .await?
        .id;
    let asset_id = AssetsManager::get_by_address(pool, withdraw.asset)
        .await?
        .id;
    Ok::<engine_base::TransferRequest, BlockchainEngineError>(engine_base::TransferRequest {
        maker_id: maker_id.to_string(),
        taker_id: Uuid::default().to_string(),
        asset_id: asset_id.to_string(),
        amount: serde_json::to_string(&withdraw.amount)?,
    })
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WithdrawInsert {
    pub owner: Address,
    pub spender: Address,
    pub asset: Address,
    pub amount: Fraction,
    pub nonce: i64,
    pub deadline: DateTime<Utc>,
    pub is_active: bool,
}
