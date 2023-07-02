use evm::address::Address;
use fraction::Fraction;
use sqlx::{postgres::PgPool, Result};

use crate::database::projections::asset::Asset;

#[derive(Debug, Clone)]
pub struct AssetsManager {
    database: PgPool,
}

impl AssetsManager {
    pub fn new(database: PgPool) -> Self {
        Self { database }
    }

    pub async fn get_by_address(&self, address: Address) -> Result<Asset> {
        sqlx::query_as!(
            Asset,
            r#"
            SELECT
                id,
                decimals as "decimals: Fraction"
            FROM assets
            WHERE address = $1
            "#,
            address.to_string()
        )
        .fetch_one(&self.database)
        .await
    }
}
