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
    async fn get_all(&self) -> Pin<Box<dyn Stream<Item = Result<Asset>> + Send + '_>> {
        sqlx::query_as!(
            Asset,
            r#"
            SELECT
                spot.assets.id,
                spot.assets.created_at,
                spot.assets.name,
                spot.assets.symbol
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
                    spot.assets.id,
                    spot.assets.created_at,
                    spot.assets.name,
                    spot.assets.symbol
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
                (id, created_at, name, symbol) VALUES ($1, $2, $3, $4)
            "#,
            element.id,
            element.created_at,
            element.name,
            element.symbol
        )
        .execute(&self.database)
        .await
    }
}
