use std::pin::Pin;

use futures::Stream;
use sqlx::{
    postgres::{PgPool, PgQueryResult},
    types::Uuid,
    Result,
};

use crate::{
    projections::spot::candlestick_metadata::{CandlestickMetadata, CandlestickType},
    traits::table_manager::TableManager,
};

#[derive(Debug, Clone)]
pub struct CandlestickMetadataManager {
    database: PgPool,
}

impl CandlestickMetadataManager {
    pub fn new(database: PgPool) -> Self {
        Self { database }
    }
}

impl TableManager<CandlestickMetadata> for CandlestickMetadataManager {
    fn get_all(&self) -> Pin<Box<dyn Stream<Item = Result<CandlestickMetadata>> + Send + '_>> {
        sqlx::query_as!(
            CandlestickMetadata,
            r#"
            SELECT
                id,
                quote_asset_id,
                base_asset_id,
                kind as "kind: CandlestickType",
                span
            FROM spot.candlesticks_metadata
            "#
        )
        .fetch(&self.database)
    }
    async fn get_by_id(&self, id: Uuid) -> Result<CandlestickMetadata> {
        sqlx::query_as!(
            CandlestickMetadata,
            r#"
            SELECT
                id,
                quote_asset_id,
                base_asset_id,
                kind as "kind: CandlestickType",
                span
            FROM spot.candlesticks_metadata
            WHERE spot.candlesticks_metadata.id = $1
            "#,
            id
        )
        .fetch_one(&self.database)
        .await
    }
    async fn insert(&self, element: CandlestickMetadata) -> Result<PgQueryResult> {
        sqlx::query!(
            r#"
            INSERT INTO 
                spot.candlesticks_metadata 
                (id, quote_asset_id, base_asset_id, kind, span)
            VALUES
                ($1, $2, $3, $4::candlestick_type, $5)
            "#,
            element.id,
            element.quote_asset_id,
            element.base_asset_id,
            element.kind as _,
            element.span
        )
        .execute(&self.database)
        .await
    }
    async fn update(&self, element: CandlestickMetadata) -> Result<PgQueryResult> {
        sqlx::query!(
            r#"
            UPDATE 
                spot.candlesticks_metadata 
            SET
                quote_asset_id = $2,
                base_asset_id = $3,
                kind = $4,
                span = $5
            WHERE
                id = $1
            "#,
            element.id,
            element.quote_asset_id,
            element.base_asset_id,
            element.kind as _,
            element.span
        )
        .execute(&self.database)
        .await
    }
    async fn delete(&self, element: CandlestickMetadata) -> Result<PgQueryResult> {
        sqlx::query!(
            r#"
            DELETE FROM
                spot.candlesticks_metadata 
            WHERE
                id = $1
            "#,
            element.id,
        )
        .execute(&self.database)
        .await
    }
}
