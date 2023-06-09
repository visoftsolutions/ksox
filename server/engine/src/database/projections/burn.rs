use fraction::Fraction;
use serde::{Deserialize, Serialize};
use sqlx::types::{
    chrono::{DateTime, Utc},
    Uuid,
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Burn {
    pub user_id: Uuid,
    pub asset_id: Uuid,
    pub amount: Fraction,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct BurnGet {
    pub last_modification_at: DateTime<Utc>,
}
