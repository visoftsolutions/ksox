use chrono::{DateTime, Utc};
use fraction::Fraction;
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Transfer {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub last_modification_at: DateTime<Utc>,
    pub from_valut_id: Uuid,
    pub from_user_id: Uuid,
    pub to_valut_id: Uuid,
    pub to_user_id: Uuid,
    pub asset_id: Uuid,
    pub amount: Fraction,
    pub fee: Fraction,
}

pub struct UserFriendlyTransfer {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub from_user_id: Uuid,
    pub from_user_name: String,
    pub to_user_id: Uuid,
    pub to_user_name: String,
    pub asset_id: Uuid,
    pub asset_icon_path: Uuid,
    pub asset_name: String,
    pub asset_symbol: String,
    pub amount: Fraction,
    pub fee: Fraction,
}
