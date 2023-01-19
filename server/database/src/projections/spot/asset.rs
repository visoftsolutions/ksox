use chrono::{DateTime, Utc};
use sqlx::types::Uuid;

pub struct Asset {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub name: String,
    pub symbol: String,
}
