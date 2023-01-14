use sqlx::types::Uuid;
use chrono::{DateTime, Utc};
use crate::managers::types::Address;

pub struct User {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub address: Address,
}
