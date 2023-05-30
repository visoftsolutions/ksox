use chrono::{DateTime, Utc};
use fraction::Fraction;
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Transfer {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub last_modification_at: DateTime<Utc>,
    pub from: Uuid,
    pub to: Uuid,
    pub asset: Uuid,
    pub amount: Fraction,
}
