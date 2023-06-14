use fraction::Fraction;
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transfer {
    pub maker_id: Uuid,
    pub taker_id: Uuid,
    pub asset_id: Uuid,
    pub amount: Fraction,
}
