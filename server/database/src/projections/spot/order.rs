use chrono::{DateTime, Utc};
use sqlx::{
    types::{BigDecimal, Uuid},
    Type,
};

#[derive(Debug, Type)]
pub enum Status {
    Active,
    PartiallyFilled,
    Filled,
    Cancelled,
}

pub struct Order {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub user_id: Uuid,
    pub status: Status,
    pub quote_asset_id: Uuid,
    pub base_asset_id: Uuid,
    pub quote_asset_volume: BigDecimal,
    pub base_asset_price: f64,
}
