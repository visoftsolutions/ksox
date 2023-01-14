use sqlx::types::Uuid;
use chrono::{DateTime, Utc};

pub struct Asset {
    id: Uuid,
    created_at: DateTime<Utc>,
    name: String,
    symbol: String,
}
