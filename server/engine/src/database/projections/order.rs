use fraction::Fraction;
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Order {
    pub id: Uuid,
    pub is_active: bool,
    pub maker_id: Uuid,
    pub quote_asset_id: Uuid,
    pub base_asset_id: Uuid,
    pub quote_asset_volume_left: Fraction,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderGet {
    pub id: Uuid,
    pub maker_id: Uuid,
    pub price: Fraction,
    pub quote_asset_volume_left: Fraction,
    pub maker_fee: Fraction,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderInsert {
    pub maker_id: Uuid,
    pub maker_presentation: bool,
    pub quote_asset_id: Uuid,
    pub base_asset_id: Uuid,
    pub price: Fraction,
    pub quote_asset_volume: Fraction,
    pub quote_asset_volume_left: Fraction,
    pub maker_fee: Fraction,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderUpdate {
    pub id: Uuid,
    pub is_active: bool,
    pub quote_asset_volume_left: Fraction,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderStatus {
    pub id: Uuid,
    pub is_active: bool,
}

impl From<Order> for OrderUpdate {
    fn from(value: Order) -> Self {
        Self {
            id: value.id,
            is_active: value.is_active,
            quote_asset_volume_left: value.quote_asset_volume_left,
        }
    }
}

impl From<Order> for OrderStatus {
    fn from(value: Order) -> Self {
        Self {
            id: value.id,
            is_active: value.is_active,
        }
    }
}
