use fraction::Fraction;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Asset {
    pub id: Uuid,
    pub decimals: Fraction,
    pub maker_fee: Fraction,
    pub taker_fee: Fraction,
}
