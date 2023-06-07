use chrono::{DateTime, Utc};
use fraction::Fraction;
use sqlx::{postgres::PgPool, Result};

use crate::database::projections::candlestick::{Candlestick, CandlestickType};

#[derive(Debug, Clone)]
pub struct CandlesticksManager {
    database: PgPool,
}

impl CandlesticksManager {
    pub fn new(database: PgPool) -> Self {
        Self { database }
    }
    pub async fn get_modified(
        &self,
        last_modification_at: DateTime<Utc>,
    ) -> Result<Vec<Candlestick>> {
        sqlx::query_as!(
            Candlestick,
            r#"
            SELECT
                id,
                created_at,
                last_modification_at,
                quote_asset_id,
                base_asset_id,
                kind as "kind: CandlestickType",
                topen,
                tclose,
                open as "open: Fraction",
                high as "high: Fraction",
                low as "low: Fraction",
                close as "close: Fraction",
                span,
                taker_quote_volume as "taker_quote_volume: Fraction",
                taker_base_volume as "taker_base_volume: Fraction",
                maker_quote_volume as "maker_quote_volume: Fraction",
                maker_base_volume as "maker_base_volume: Fraction"
            FROM spot.candlesticks
            WHERE last_modification_at > $1
            ORDER BY last_modification_at ASC
            "#,
            last_modification_at
        )
        .fetch_all(&self.database)
        .await
    }
}
