use chrono::{DateTime, Utc};
use evm::address::Address;
use fraction::Fraction;
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Asset {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub last_modification_at: DateTime<Utc>,
    pub name: String,
    pub icon_path: String,
    pub symbol: String,
    pub address: Address,
    pub decimals: Fraction,
    pub maker_fee: Fraction,
    pub taker_fee: Fraction,
    pub transfer_fee: Fraction,
}
