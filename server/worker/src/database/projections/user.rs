use std::str::FromStr;

use chrono::{DateTime, Utc};
use evm::address::Address;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, types::Uuid, FromRow, Row};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub last_modification_at: DateTime<Utc>,
    pub address: Address,
    pub name: Option<String>
}

impl FromRow<'_, PgRow> for User {
    fn from_row(row: &PgRow) -> sqlx::Result<Self> {
        let address =
            Address::from_str(&row.try_get::<'_, String, _>("address")?).map_err(|e| {
                sqlx::Error::Decode(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    e.to_string(),
                )))
            })?;
        Ok(User {
            id: row.try_get("id")?,
            created_at: row.try_get("created_at")?,
            last_modification_at: row.try_get("last_modification_at")?,
            address,
            name: row.try_get("name")?,
        })
    }
}
