use std::pin::Pin;

use bigdecimal::BigDecimal;
use futures::Stream;
use sqlx::{
    postgres::{PgPool, PgQueryResult},
    types::Uuid,
    Result,
};

use crate::{
    projections::spot::candlestick::Candlestick,
    traits::table_manager::TableManager,
    types::{CandlestickType, Fraction, Volume},
};

#[derive(Debug, Clone)]
pub struct CandlestickManager {
    database: PgPool,
}

impl CandlestickManager {
    pub fn new(database: PgPool) -> Self {
        Self { database }
    }
}

impl TableManager<Candlestick> for CandlestickManager {
    fn get_all(&self) -> Pin<Box<dyn Stream<Item = Result<Candlestick>> + Send + '_>> {
        sqlx::query_as!(
            Candlestick,
            r#"
            SELECT
                id,
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
                taker_quote_volume as "taker_quote_volume: Volume",
                taker_base_volume as "taker_base_volume: Volume",
                maker_quote_volume as "maker_quote_volume: Volume",
                maker_base_volume as "maker_base_volume: Volume"
            FROM spot.candlesticks
            "#
        )
        .fetch(&self.database)
    }
    async fn get_by_id(&self, id: Uuid) -> Result<Candlestick> {
        sqlx::query_as!(
            Candlestick,
            r#"
            SELECT
                id,
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
                taker_quote_volume as "taker_quote_volume: Volume",
                taker_base_volume as "taker_base_volume: Volume",
                maker_quote_volume as "maker_quote_volume: Volume",
                maker_base_volume as "maker_base_volume: Volume"
            FROM spot.candlesticks
            WHERE spot.candlesticks.id = $1
            "#,
            id
        )
        .fetch_one(&self.database)
        .await
    }
    async fn insert(&self, element: Candlestick) -> Result<PgQueryResult> {
        let taker_quote_volume: BigDecimal = element.taker_quote_volume.into();
        let taker_base_volume: BigDecimal = element.taker_base_volume.into();
        let maker_quote_volume: BigDecimal = element.maker_quote_volume.into();
        let maker_base_volume: BigDecimal = element.maker_base_volume.into();
        sqlx::query!(
            r#"
            INSERT INTO 
                spot.candlesticks 
                (
                    id,
                    quote_asset_id,
                    base_asset_id,
                    kind,
                    topen,
                    tclose,
                    open,
                    high,
                    low,
                    close,
                    span,
                    taker_quote_volume,
                    taker_base_volume,
                    maker_quote_volume,
                    maker_base_volume    
                )
            VALUES
                (
                    $1,
                    $2,
                    $3,
                    $4::candlestick_type,
                    $5,
                    $6,
                    $7::fraction,
                    $8::fraction,
                    $9::fraction,
                    $10::fraction,
                    $11,
                    $12,
                    $13,
                    $14,
                    $15
                )
            "#,
            element.id,
            element.quote_asset_id,
            element.base_asset_id,
            element.kind as _,
            element.topen,
            element.tclose,
            element.open.to_string() as _,
            element.high.to_string() as _,
            element.low.to_string() as _,
            element.close.to_string() as _,
            element.span,
            taker_quote_volume,
            taker_base_volume,
            maker_quote_volume,
            maker_base_volume
        )
        .execute(&self.database)
        .await
    }
    async fn update(&self, element: Candlestick) -> Result<PgQueryResult> {
        let taker_quote_volume: BigDecimal = element.taker_quote_volume.into();
        let taker_base_volume: BigDecimal = element.taker_base_volume.into();
        let maker_quote_volume: BigDecimal = element.maker_quote_volume.into();
        let maker_base_volume: BigDecimal = element.maker_base_volume.into();
        sqlx::query!(
            r#"
            UPDATE 
                spot.candlesticks 
            SET
                quote_asset_id = $2,
                base_asset_id = $3,
                kind = $4,
                topen = $5,
                tclose = $6,
                open = $7,
                high = $8,
                low = $9,
                close = $10,
                span = $11,
                taker_quote_volume = $12,
                taker_base_volume = $13,
                maker_quote_volume = $14,
                maker_base_volume = $15
            WHERE
                id = $1
            "#,
            element.id,
            element.quote_asset_id,
            element.base_asset_id,
            element.kind as _,
            element.topen,
            element.tclose,
            element.open.to_string() as _,
            element.high.to_string() as _,
            element.low.to_string() as _,
            element.close.to_string() as _,
            element.span,
            taker_quote_volume,
            taker_base_volume,
            maker_quote_volume,
            maker_base_volume
        )
        .execute(&self.database)
        .await
    }
    async fn delete(&self, element: Candlestick) -> Result<PgQueryResult> {
        sqlx::query!(
            r#"
            DELETE FROM
                spot.candlesticks 
            WHERE
                id = $1
            "#,
            element.id,
        )
        .execute(&self.database)
        .await
    }
}