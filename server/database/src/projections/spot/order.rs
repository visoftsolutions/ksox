use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{types::Uuid, Type};

use crate::types::Volume;

#[derive(Debug, Type, Serialize, Deserialize, Clone, PartialEq)]
#[sqlx(type_name = "products_status")]
pub enum Status {
    #[serde(rename = "active")]
    #[sqlx(rename = "active")]
    Active,
    #[serde(rename = "partially_filled")]
    #[sqlx(rename = "partially_filled")]
    PartiallyFilled,
    #[serde(rename = "filled")]
    #[sqlx(rename = "filled")]
    Filled,
    #[serde(rename = "cancelled")]
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
    pub quote_asset_volume: Volume,
    pub base_asset_volume: Volume,
}
