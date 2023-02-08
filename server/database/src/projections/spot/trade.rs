use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

use crate::types::Volume;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Trade {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub taker_id: Uuid,
    pub order_id: Uuid,
    pub taker_quote_volume: Volume,
    pub maker_quote_volume: Volume,
    pub taker_base_volume: Volume,
    pub maker_base_volume: Volume,
}
