use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use value::Value;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Valut {
    pub id: Uuid,
    pub balance: Value,
}
