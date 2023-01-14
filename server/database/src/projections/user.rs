use crate::managers::types::EvmAddress;
use chrono::{DateTime, Utc};
use sqlx::types::Uuid;

#[derive(Debug)]
pub struct User {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub address: EvmAddress,
}
