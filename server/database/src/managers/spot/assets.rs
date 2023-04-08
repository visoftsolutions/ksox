use std::pin::Pin;

use futures::Stream;
use sqlx::{
    postgres::{PgPool, PgQueryResult},
    types::Uuid,
    Result,
};

use crate::{
    projections::spot::asset::Asset,
    traits::{get_modified::GetModified, table_manager::TableManager},
    types::Fraction,
};

#[derive(Debug, Clone)]
pub struct AssetsManager {
    database: PgPool,
}

impl AssetsManager {
    pub fn new(database: PgPool) -> Self {
        Self { database }
    }
}

impl TableManager<Asset> for AssetsManager {
    fn get_all(&self) -> Pin<Box<dyn Stream<Item = Result<Asset>> + Send + '_>> {
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
            "#
        )
        .fetch(&self.database)
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Asset> {
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
            WHERE spot.assets.id = $1
            "#,
            id
        )
        .fetch_one(&self.database)
        .await
    }

    async fn insert(&self, element: Asset) -> Result<PgQueryResult> {
        sqlx::query!(
            r#"
            INSERT INTO 
                spot.assets 
                (id, created_at, name, symbol, maker_fee, taker_fee)
            VALUES
                ($1, $2, $3, $4, $5::fraction, $6::fraction)
            "#,
            element.id,
            element.created_at,
            element.name,
            element.symbol,
            element.maker_fee.to_string() as _,
            element.taker_fee.to_string() as _
        )
        .execute(&self.database)
        .await
    }

    async fn update(&self, element: Asset) -> Result<PgQueryResult> {
        sqlx::query!(
            r#"
            UPDATE 
                spot.assets 
            SET
                name = $2,
                symbol = $3,
                maker_fee = $4,
                taker_fee = $5
            WHERE
                id = $1
            "#,
            element.id,
            element.name,
            element.symbol,
            element.maker_fee.to_string() as _,
            element.taker_fee.to_string() as _
        )
        .execute(&self.database)
        .await
    }

    async fn delete(&self, element: Asset) -> Result<PgQueryResult> {
        sqlx::query!(
            r#"
            DELETE FROM 
                spot.assets 
            WHERE
                id = $1
            "#,
            element.id,
        )
        .execute(&self.database)
        .await
    }
}

impl GetModified<Asset> for AssetsManager {
    async fn get_modified(
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
            WHERE spot.assets.last_modification_at > $1
            "#,
            last_modification_at
        )
        .fetch_all(&self.database)
        .await
    }
}
