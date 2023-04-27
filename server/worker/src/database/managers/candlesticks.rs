use chrono::{DateTime, Utc};
use fraction::Fraction;
use sqlx::{
    postgres::{PgPool, PgQueryResult},
    types::Uuid,
    Result,
};

use super::notifications::NotificationManagerSubscriber;
use crate::database::projections::candlestick::{Candlestick, CandlestickType};

#[derive(Debug, Clone)]
pub struct CandlesticksManager {
    database: PgPool,
}

impl CandlesticksManager {
    pub fn new(database: PgPool) -> Self {
        Self { database }
    }
    pub async fn get_interval_for_asset_pair(
        &self,
        quote_asset_id: Uuid,
        base_asset_id: Uuid,
        topen: DateTime<Utc>,
        tclose: DateTime<Utc>,
    ) -> Result<Option<Candlestick>> {
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
            WHERE spot.candlesticks.quote_asset_id = $1 AND spot.candlesticks.base_asset_id = $2
            AND spot.candlesticks.topen = $3 AND spot.candlesticks.tclose = $4
            "#,
            quote_asset_id,
            base_asset_id,
            topen,
            tclose
        )
        .fetch_optional(&self.database)
        .await
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

    pub async fn insert(&self, element: Candlestick) -> Result<PgQueryResult> {
        let now = Utc::now();
        sqlx::query!(
            r#"
            INSERT INTO 
                spot.candlesticks 
                (
                    id,
                    created_at,
                    last_modification_at,
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
                    $4,
                    $5,
                    $6::candlestick_type,
                    $7,
                    $8,
                    $9::fraction,
                    $10::fraction,
                    $11::fraction,
                    $12::fraction,
                    $13,
                    $14::fraction,
                    $15::fraction,
                    $16::fraction,
                    $17::fraction
                )
            "#,
            element.id,
            now,
            now,
            element.quote_asset_id,
            element.base_asset_id,
            element.kind as _,
            element.topen,
            element.tclose,
            element.open.to_tuple_string() as _,
            element.high.to_tuple_string() as _,
            element.low.to_tuple_string() as _,
            element.close.to_tuple_string() as _,
            element.span,
            element.taker_quote_volume.to_tuple_string() as _,
            element.taker_base_volume.to_tuple_string() as _,
            element.maker_quote_volume.to_tuple_string() as _,
            element.maker_base_volume.to_tuple_string() as _
        )
        .execute(&self.database)
        .await
    }
}

#[derive(Debug, Clone)]
pub struct CandlesticksNotificationManager {
    notification_manager_subscriber: NotificationManagerSubscriber,
}
impl CandlesticksNotificationManager {
    pub fn new(notification_manager_subscriber: NotificationManagerSubscriber) -> Self {
        Self {
            notification_manager_subscriber,
        }
    }
}
