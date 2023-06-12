use chrono::{DateTime, Utc};
use fraction::Fraction;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Trade {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub last_modification_at: DateTime<Utc>,
    pub quote_asset_id: Uuid,
    pub base_asset_id: Uuid,
    pub taker_id: Uuid,
    pub taker_presentation: bool,
    pub maker_id: Uuid,
    pub maker_presentation: bool,
    pub price: Fraction,
    pub taker_quote_volume: Fraction,
    pub taker_base_volume: Fraction,
    pub maker_quote_volume: Fraction,
    pub maker_base_volume: Fraction,
}
