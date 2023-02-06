use std::pin::Pin;

use futures::Stream;
use sqlx::{
    postgres::{PgPool, PgQueryResult},
    types::Uuid,
    Result,
};

use crate::{projections::spot::asset::Asset, traits::manager::Manager};

#[derive(Debug, Clone)]
pub struct AssetsManager {
    database: PgPool,
}

impl AssetsManager {
    pub fn new(database: PgPool) -> Self {
        AssetsManager { database }
    }
}

impl Manager<Asset> for AssetsManager {
    fn get_all(&self) -> Pin<Box<dyn Stream<Item = Result<Asset>> + Send + '_>> {
        sqlx::query_as!(
            Asset,
            r#"
            SELECT
                id,
                created_at,
                name,
                symbol,
                maker_fee,
                taker_fee
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
                name,
                symbol,
                maker_fee,
                taker_fee
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
                ($1, $2, $3, $4, $5, $6)
            "#,
            element.id,
            element.created_at,
            element.name,
            element.symbol,
            element.maker_fee,
            element.taker_fee
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
                created_at = $2,
                name = $3,
                symbol = $4,
                maker_fee = $5,
                taker_fee = $6
            WHERE
                id = $1
            "#,
            element.id,
            element.created_at,
            element.name,
            element.symbol,
            element.maker_fee,
            element.taker_fee
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
