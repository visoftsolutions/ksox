use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

use crate::types::Volume;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Order {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub user_id: Uuid,
    pub is_active: bool,
    pub quote_asset_id: Uuid,
    pub base_asset_id: Uuid,
    pub quote_asset_volume: Volume,
    pub base_asset_volume: Volume,
    pub quote_asset_volume_left: Volume,
}
