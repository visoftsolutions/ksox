use chrono::{DateTime, Utc};
use evm::{address::Address, txhash::TxHash};
use fraction::Fraction;
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Deposit {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub last_modification_at: DateTime<Utc>,
    pub maker_address: Address,
    pub taker_address: Address,
    pub asset_address: Address,
    pub tx_hash: TxHash,
    pub amount: Fraction,
    pub confirmations: Fraction,
}
