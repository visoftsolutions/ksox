use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

use crate::types::Fraction;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Trade {
    pub taker_id: Uuid,
    pub order_id: Uuid,
    pub taker_quote_volume: Fraction,
    pub taker_base_volume: Fraction,
    pub maker_quote_volume: Fraction,
    pub maker_base_volume: Fraction,
}
