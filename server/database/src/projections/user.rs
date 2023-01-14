use crate::managers::types::Address;
use chrono::{DateTime, Utc};
use sqlx::types::Uuid;

pub struct User {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub address: Address,
}
