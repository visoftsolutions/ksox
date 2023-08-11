use evm::address::Address;
use fraction::Fraction;
use sqlx::{Postgres, Result, Transaction};
use uuid::Uuid;

use crate::database::projections::asset::Asset;

#[derive(Debug, Clone)]
pub struct AssetsManager {}

impl AssetsManager {
    pub async fn get_by_address<'t>(
        t: &'t mut Transaction<'_, Postgres>,
        address: &Address,
    ) -> Result<Asset> {
        sqlx::query_as!(
            Asset,
            r#"
            SELECT
                id,
                address as "address: Address",
                decimals as "decimals: Fraction"
            FROM assets
            WHERE address = $1
            "#,
            address.to_string()
        )
        .fetch_one(t.as_mut())
        .await
    }

    pub async fn get_by_id<'t>(t: &'t mut Transaction<'_, Postgres>, id: Uuid) -> Result<Asset> {
        sqlx::query_as!(
            Asset,
            r#"
            SELECT
                id,
                address as "address: Address",
                decimals as "decimals: Fraction"
            FROM assets
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(t.as_mut())
        .await
    }
}
