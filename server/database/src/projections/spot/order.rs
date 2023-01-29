use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{
    types::{BigDecimal, Uuid},
    Type,
};

#[derive(Debug, Type, Serialize, Deserialize, Clone)]
#[sqlx(type_name = "products_status")]
pub enum Status {
    #[sqlx(rename = "active")]
    Active,
    #[sqlx(rename = "partially_filled")]
    PartiallyFilled,
    #[sqlx(rename = "filled")]
    Filled,
    #[sqlx(rename = "cancelled")]
    Cancelled,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
