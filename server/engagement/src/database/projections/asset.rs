use chrono::{DateTime, Utc};
use fraction::Fraction;
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Asset {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub last_modification_at: DateTime<Utc>,
    pub name: String,
    pub symbol: String,
    pub maker_fee: Fraction,
    pub taker_fee: Fraction,
}
