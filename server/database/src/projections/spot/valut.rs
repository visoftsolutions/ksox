use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

use crate::types::Volume;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Valut {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub last_modification_at: DateTime<Utc>,
    pub user_id: Uuid,
    pub asset_id: Uuid,
    pub balance: Volume,
}
