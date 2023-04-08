use std::str::FromStr;

use chrono::{DateTime, Utc};
use sqlx::{postgres::PgRow, types::Uuid, FromRow, Row};

use crate::{
    managers::spot::valuts::ValutsManager,
    traits::TableManager,
    types::{EvmAddress, Volume},
};

#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub last_modification_at: DateTime<Utc>,
    pub address: EvmAddress,
}

impl FromRow<'_, PgRow> for User {
    fn from_row(row: &PgRow) -> sqlx::Result<Self> {
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
            last_modification_at: row.try_get("last_modification_at")?,
            address: evm_address,
        })
    }
}

impl User {
    pub async fn mint(
        &self,
        valuts_manager: &ValutsManager,
        asset_id: Uuid,
        amount: Volume,
    ) -> Result<(), sqlx::Error> {
        let mut valut = valuts_manager
            .get_or_create_for_user_asset(self.id, asset_id)
            .await?;
        valut.balance += amount;
        valuts_manager.update(valut).await?;
        Ok(())
    }

    pub async fn burn(
        &self,
        valuts_manager: &ValutsManager,
        asset_id: Uuid,
        amount: Volume,
    ) -> Result<(), sqlx::Error> {
        let mut valut = valuts_manager.get_for_user_asset(self.id, asset_id).await?;
        valut.balance -= amount;
        valuts_manager.update(valut).await?;
        Ok(())
    }
}
