use chrono::{DateTime, Utc};
use sqlx::types::Uuid;

pub struct Badge {
    pub id: Uuid,
    pub name: String,
    pub family: String,
    pub description: String,
    pub value: u64,
    pub created_at: DateTime<Utc>,
}
