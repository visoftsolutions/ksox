use fraction::Fraction;
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Valut {
    pub id: Uuid,
    pub balance: Fraction,
}