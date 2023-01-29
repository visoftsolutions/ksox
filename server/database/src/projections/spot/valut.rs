use serde::{Deserialize, Serialize};
use sqlx::types::{BigDecimal, Uuid};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Valut {
    pub id: Uuid,
    pub user_id: Uuid,
    pub asset_id: Uuid,
    pub balance: BigDecimal,
}
