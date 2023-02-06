use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Asset {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub name: String,
    pub symbol: String,
    pub maker_fee: BigDecimal,
    pub taker_fee: BigDecimal,
}
