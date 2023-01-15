use futures::Stream;
use sqlx::{postgres::PgPool, types::Uuid, Result};
use std::pin::Pin;

use crate::projections::spot::asset::Asset;

struct AssetsManager {
    database: PgPool,
}

impl AssetsManager {
    pub fn new(database: PgPool) -> Self {
        AssetsManager { database }
    }

    pub async fn get_all(&self) -> Pin<Box<dyn Stream<Item = Result<Asset>> + Send + '_>> {
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

    pub async fn get_by_id(
        &self,
        id: Uuid,
    ) -> Pin<Box<dyn Stream<Item = Result<Asset>> + Send + '_>> {
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
        .fetch(&self.database)
    }
}
