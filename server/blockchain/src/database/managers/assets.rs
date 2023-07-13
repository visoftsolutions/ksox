use evm::address::Address;
use fraction::Fraction;
use sqlx::{Postgres, Result, Transaction};
use uuid::Uuid;

use crate::database::projections::asset::Asset;

#[derive(Debug, Clone)]
pub struct AssetsManager {}

impl AssetsManager {
    pub async fn get_by_address<'t, 'p>(
        pool: &'t mut Transaction<'p, Postgres>,
        address: Address,
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
        .fetch_one(pool)
        .await
    }

    pub async fn get_by_id<'t, 'p>(
        pool: &'t mut Transaction<'p, Postgres>,
        id: Uuid,
    ) -> Result<Asset> {
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
        .fetch_one(pool)
        .await
    }
}
