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

    pub async fn get_modified(
        &self,
        last_modification_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<Vec<Asset>> {
        sqlx::query_as!(
            Asset,
            r#"
            SELECT
                id,
                created_at,
                last_modification_at,
                name,
                symbol,
                maker_fee as "maker_fee: Fraction",
                taker_fee as "taker_fee: Fraction"
            FROM spot.assets
            WHERE last_modification_at > $1
            ORDER BY last_modification_at ASC
            "#,
            last_modification_at
        )
        .fetch_all(&self.database)
        .await
    }
}
