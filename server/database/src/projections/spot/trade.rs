use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::{BigDecimal, Uuid};

#[derive(Debug, Serialize, Deserialize)]
pub struct Trade {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub taker_id: Uuid,
    pub order_id: Uuid,
    pub taker_quote_volume: BigDecimal,
    pub maker_quote_volume: BigDecimal,
    pub taker_base_volume: BigDecimal,
    pub maker_base_volume: BigDecimal,
}
