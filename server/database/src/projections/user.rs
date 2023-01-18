use std::str::FromStr;

use crate::managers::types::EvmAddress;
use chrono::{DateTime, Utc};
use sqlx::{postgres::PgRow, types::Uuid, FromRow, Result, Row};

#[derive(Debug)]
pub struct User {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub address: EvmAddress,
}

impl FromRow<'_, PgRow> for User {
    fn from_row(row: &PgRow) -> Result<Self> {
        let evm_address =
            EvmAddress::from_str(&row.try_get::<'_, String, _>("address")?).map_err(|e| {
                sqlx::Error::Decode(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    e.to_string(),
                )))
            })?;
        Ok(User {
            id: row.try_get("id")?,
            created_at: row.try_get("created_at")?,
            address: evm_address,
        })
    }
}
