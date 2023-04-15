use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

use crate::types::Fraction;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Valut {
    pub id: Uuid,
    pub balance: Fraction,
}
