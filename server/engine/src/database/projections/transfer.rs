use fraction::Fraction;
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transfer {
    pub from_valut_id: Uuid,
    pub to_valut_id: Uuid,
    pub fee_valut_id: Uuid,
    pub asset_id: Uuid,
    pub amount: Fraction,
    pub fee: Fraction,
}
