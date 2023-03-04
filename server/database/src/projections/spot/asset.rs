use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

use crate::types::Volume;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Asset {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub name: String,
    pub symbol: String,
    pub maker_fee_num: Volume,
    pub maker_fee_denum: Volume,
    pub taker_fee_num: Volume,
    pub taker_fee_denum: Volume,
}
