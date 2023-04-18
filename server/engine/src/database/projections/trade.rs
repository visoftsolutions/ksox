use fraction::Fraction;
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Trade {
    pub quote_asset_id: Uuid,
    pub base_asset_id: Uuid,
    pub taker_id: Uuid,
    pub order_id: Uuid,
    pub taker_price: Fraction,
    pub taker_quote_volume: Fraction,
    pub taker_base_volume: Fraction,
    pub maker_quote_volume: Fraction,
    pub maker_base_volume: Fraction,
}
