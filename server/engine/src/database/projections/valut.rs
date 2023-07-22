use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use value::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Valut {
    pub id: Uuid,
    pub balance: Value,
}
