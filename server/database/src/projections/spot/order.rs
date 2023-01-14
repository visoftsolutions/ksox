use sqlx::types::Uuid;
use chrono::{DateTime, Utc};
use ethereum_types::U256;

pub enum Status {
    Active,
    PartiallyFilled,
    Filled,
    Cancelled,
}

pub struct Order {
    id: Uuid,
    created_at: DateTime<Utc>,
    user_id: Uuid,
    status: Status,
    quote_asset_id: Uuid,
    base_asset_id: Uuid,
    quote_asset_volume: U256,
    base_asset_price: f64,
}
