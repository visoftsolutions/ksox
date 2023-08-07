use chrono::{DateTime, Utc};
use ethereum_types::U256;
use ethers::prelude::LogMeta;
use evm::{address::Address, txhash::TxHash};
use fraction::{num_traits::One, Fraction};
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

use crate::{
    blockchain_engine::models::BlockchainEngineError,
    contracts::treasury::WithdrawFilter,
    database::managers::{assets::AssetsManager, users::UsersManager},
    engine_base,
};
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Withdraw {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub last_modification_at: DateTime<Utc>,
    pub maker_address: Address,
    pub taker_address: Address,
    pub asset_address: Address,
    pub amount: Fraction,
    pub deadline: DateTime<Utc>,
}
pub async fn withdraw_to_transfer_request(
    pool: &mut Transaction<'_, Postgres>,
    withdraw: Withdraw,
) -> Result<engine_base::TransferRequest, BlockchainEngineError> {
    let maker_id = UsersManager::get_by_address(pool, withdraw.maker_address)
        .await?
        .id;
    let asset_id = AssetsManager::get_by_address(pool, withdraw.asset_address)
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
    pub maker_address: Address,
    pub taker_address: Address,
    pub asset_address: Address,
    pub amount: Fraction,
    pub deadline: DateTime<Utc>,
}
