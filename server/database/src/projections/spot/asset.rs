use chrono::{DateTime, Utc};
use sqlx::types::Uuid;

pub struct Asset {
    id: Uuid,
    created_at: DateTime<Utc>,
    name: String,
    symbol: String,
}
