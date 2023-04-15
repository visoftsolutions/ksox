use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::types::Fraction;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Asset {
    pub id: Uuid,
    pub maker_fee: Fraction,
    pub taker_fee: Fraction,
}
