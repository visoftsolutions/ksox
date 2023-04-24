use chrono::{DateTime, Utc};
use fraction::Fraction;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Order {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub last_modification_at: DateTime<Utc>,
    pub user_id: Uuid,
    pub is_active: bool,
    pub quote_asset_id: Uuid,
    pub base_asset_id: Uuid,
    pub price: Fraction,
    pub quote_asset_volume: Fraction,
    pub quote_asset_volume_left: Fraction,
    pub maker_fee: Fraction,
}
