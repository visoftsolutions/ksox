use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

use crate::types::Volume;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Trade {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub quote_asset_id: Uuid,
    pub base_asset_id: Uuid,
    pub taker_id: Uuid,
    pub order_id: Uuid,
    pub taker_quote_volume: Volume,
    pub taker_base_volume: Volume,
    pub maker_quote_volume: Volume,
    pub maker_base_volume: Volume,
}

impl Trade {
    pub fn is_opposite(&self, quote_asset_id: Uuid, base_asset_id: Uuid) -> bool {
        self.quote_asset_id == base_asset_id && self.base_asset_id == quote_asset_id
    }
    pub fn inverse(self) -> Self {
        Self {
            id: self.id,
            created_at: self.created_at,
            quote_asset_id: self.base_asset_id,
            base_asset_id: self.quote_asset_id,
            taker_id: self.taker_id,
            order_id: self.order_id,
            taker_quote_volume: self.taker_base_volume,
            taker_base_volume: self.taker_quote_volume,
            maker_quote_volume: self.maker_base_volume,
            maker_base_volume: self.maker_quote_volume,
        }
    }
}
