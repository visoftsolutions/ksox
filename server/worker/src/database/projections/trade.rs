use chrono::{DateTime, Utc};
use fraction::{num_traits::Inv, Fraction};
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
    pub order_id: Uuid,
    pub price: Fraction,
    pub taker_quote_volume: Fraction,
    pub taker_base_volume: Fraction,
    pub maker_quote_volume: Fraction,
    pub maker_base_volume: Fraction,
}

impl Trade {
    pub fn is_opposite(&self, quote_asset_id: Uuid, base_asset_id: Uuid) -> bool {
        self.quote_asset_id == base_asset_id && self.base_asset_id == quote_asset_id
    }
    pub fn inverse(self) -> Self {
        Self {
            id: self.id,
            created_at: self.created_at,
            last_modification_at: self.last_modification_at,
            quote_asset_id: self.base_asset_id,
            base_asset_id: self.quote_asset_id,
            taker_id: self.taker_id,
            order_id: self.order_id,
            price: self.price.inv(),
            taker_quote_volume: self.taker_base_volume,
            taker_base_volume: self.taker_quote_volume,
            maker_quote_volume: self.maker_base_volume,
            maker_base_volume: self.maker_quote_volume,
        }
    }
}