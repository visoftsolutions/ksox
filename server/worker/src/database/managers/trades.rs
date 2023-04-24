use std::pin::Pin;

use chrono::{DateTime, Utc};
use fraction::Fraction;
use futures::Stream;
use sqlx::PgPool;
use thiserror::Error;
use uuid::Uuid;

use super::notifications::{
    NotificationManagerOutput, NotificationManagerPredicateInput, NotificationManagerSubscriber,
};
use crate::database::projections::trade::Trade;

#[derive(Debug, Clone)]
pub struct TradesManager {
    database: PgPool,
}

impl TradesManager {
    pub fn new(database: PgPool) -> Self {
        TradesManager { database }
    }

    pub async fn get_modified(
        &self,
        last_modification_at: DateTime<Utc>,
    ) -> sqlx::Result<Vec<Trade>> {
        sqlx::query_as!(
            Trade,
            r#"
            SELECT
                id,
                created_at,
                last_modification_at,
                quote_asset_id,
                base_asset_id,
                taker_id,
                order_id,
                price as "price: Fraction",
                taker_quote_volume as "taker_quote_volume: Fraction",
                taker_base_volume as "taker_base_volume: Fraction",
                maker_quote_volume as "maker_quote_volume: Fraction",
                maker_base_volume as "maker_base_volume: Fraction"
            FROM spot.trades
            WHERE last_modification_at > $1
            "#,
            last_modification_at
        )
        .fetch_all(&self.database)
        .await
    }

    pub fn get_for_taker(
        &self,
        taker_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Pin<Box<dyn Stream<Item = sqlx::Result<Trade>> + Send + '_>> {
        sqlx::query_as!(
            Trade,
            r#"
            SELECT
                id,
                created_at,
                last_modification_at,
                quote_asset_id,
                base_asset_id,
                taker_id,
                order_id,
                price as "price: Fraction",
                taker_quote_volume as "taker_quote_volume: Fraction",
                taker_base_volume as "taker_base_volume: Fraction",
                maker_quote_volume as "maker_quote_volume: Fraction",
                maker_base_volume as "maker_base_volume: Fraction"
            FROM spot.trades
            WHERE taker_id = $1
            ORDER BY created_at DESC
            LIMIT $2
            OFFSET $3
            "#,
            taker_id,
            limit,
            offset
        )
        .fetch(&self.database)
    }

    pub fn get_for_asset_pair(
        &self,
        quote_asset_id: Uuid,
        base_asset_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Pin<Box<dyn Stream<Item = sqlx::Result<Trade>> + Send + '_>> {
        sqlx::query_as!(
            Trade,
            r#"
            SELECT
                id,
                created_at,
                last_modification_at,
                quote_asset_id,
                base_asset_id,
                taker_id,
                order_id,
                price as "price: Fraction",
                taker_quote_volume as "taker_quote_volume: Fraction",
                taker_base_volume as "taker_base_volume: Fraction",
                maker_quote_volume as "maker_quote_volume: Fraction",
                maker_base_volume as "maker_base_volume: Fraction"
            FROM spot.trades
            WHERE (quote_asset_id = $1 AND base_asset_id = $2) OR (quote_asset_id = $2 AND base_asset_id = $1)
            ORDER BY created_at DESC
            LIMIT $3
            OFFSET $4
            "#,
            quote_asset_id,
            base_asset_id,
            limit,
            offset
        )
        .fetch(&self.database)
    }

    pub async fn get_last_for_asset_pair(
        &self,
        quote_asset_id: Uuid,
        base_asset_id: Uuid,
    ) -> sqlx::Result<Option<Trade>> {
        sqlx::query_as!(
            Trade,
            r#"
            SELECT
                id,
                created_at,
                last_modification_at,
                quote_asset_id,
                base_asset_id,
                taker_id,
                order_id,
                price as "price: Fraction",
                taker_quote_volume as "taker_quote_volume: Fraction",
                taker_base_volume as "taker_base_volume: Fraction",
                maker_quote_volume as "maker_quote_volume: Fraction",
                maker_base_volume as "maker_base_volume: Fraction"
            FROM spot.trades
            WHERE (quote_asset_id = $1 AND base_asset_id = $2) OR (quote_asset_id = $2 AND base_asset_id = $1)
            ORDER BY created_at DESC
            LIMIT 1
            "#,
            quote_asset_id,
            base_asset_id
        )
        .fetch_optional(&self.database)
        .await
    }

    pub fn get_after_for_asset_pair(
        &self,
        quote_asset_id: Uuid,
        base_asset_id: Uuid,
        time: DateTime<Utc>,
    ) -> Pin<Box<dyn Stream<Item = sqlx::Result<Trade>> + Send + '_>> {
        sqlx::query_as!(
            Trade,
            r#"
            SELECT
                id,
                created_at,
                last_modification_at,
                quote_asset_id,
                base_asset_id,
                taker_id,
                order_id,
                price as "price: Fraction",
                taker_quote_volume as "taker_quote_volume: Fraction",
                taker_base_volume as "taker_base_volume: Fraction",
                maker_quote_volume as "maker_quote_volume: Fraction",
                maker_base_volume as "maker_base_volume: Fraction"
            FROM spot.trades
            WHERE ((quote_asset_id = $1 AND base_asset_id = $2) OR (quote_asset_id = $2 AND base_asset_id = $1)) AND created_at >= $3
            ORDER BY created_at
            "#,
            quote_asset_id,
            base_asset_id,
            time,
        )
        .fetch(&self.database)
    }

    pub fn get_between_for_asset_pair(
        &self,
        quote_asset_id: Uuid,
        base_asset_id: Uuid,
        topen: DateTime<Utc>,
        tclose: DateTime<Utc>,
    ) -> Pin<Box<dyn Stream<Item = sqlx::Result<Trade>> + Send + '_>> {
        sqlx::query_as!(
            Trade,
            r#"
            SELECT
                id,
                created_at,
                last_modification_at,
                quote_asset_id,
                base_asset_id,
                taker_id,
                order_id,
                price as "price: Fraction",
                taker_quote_volume as "taker_quote_volume: Fraction",
                taker_base_volume as "taker_base_volume: Fraction",
                maker_quote_volume as "maker_quote_volume: Fraction",
                maker_base_volume as "maker_base_volume: Fraction"
            FROM spot.trades
            WHERE ((quote_asset_id = $1 AND base_asset_id = $2) OR (quote_asset_id = $2 AND base_asset_id = $1)) AND created_at >= $3 AND created_at < $4
            "#,
            quote_asset_id,
            base_asset_id,
            topen,
            tclose,
        )
        .fetch(&self.database)
    }
}

#[derive(Debug, Error)]
pub enum TradesNotificationManagerError {}

#[derive(Debug, Clone)]
pub struct TradesNotificationManager {
    notification_manager_subscriber: NotificationManagerSubscriber,
}
impl TradesNotificationManager {
    pub fn new(notification_manager_subscriber: NotificationManagerSubscriber) -> Self {
        Self {
            notification_manager_subscriber,
        }
    }

    pub async fn subscribe_to_asset_pair(
        &self,
        quote_asset_id: Uuid,
        base_asset_id: Uuid,
    ) -> sqlx::Result<Pin<Box<dyn Stream<Item = Vec<Trade>> + Send>>> {
        let p = predicates::function::function(move |input: &NotificationManagerPredicateInput| {
            match input {
                NotificationManagerPredicateInput::SpotTradesChanged(trade) => {
                    (trade.quote_asset_id == quote_asset_id && trade.base_asset_id == base_asset_id)
                        || (trade.quote_asset_id == base_asset_id
                            && trade.base_asset_id == quote_asset_id)
                }
                _ => false,
            }
        });

        if let Ok(mut rx) = self
            .notification_manager_subscriber
            .subscribe_to(Box::new(p))
            .await
        {
            let stream = async_stream::stream! {
                while let Some(notification) = rx.recv().await {
                    if let NotificationManagerOutput::SpotTradesChanged(trades) = notification {
                        yield trades;
                    }
                }
            };
            Ok(Box::pin(stream))
        } else {
            Err(sqlx::Error::RowNotFound)
        }
    }

    pub async fn subscribe_to_taker(
        &self,
        taker_id: Uuid,
    ) -> sqlx::Result<Pin<Box<dyn Stream<Item = Vec<Trade>> + Send>>> {
        let p = predicates::function::function(move |input: &NotificationManagerPredicateInput| {
            match input {
                NotificationManagerPredicateInput::SpotTradesChanged(trade) => {
                    trade.taker_id == taker_id
                }
                _ => false,
            }
        });

        if let Ok(mut rx) = self
            .notification_manager_subscriber
            .subscribe_to(Box::new(p))
            .await
        {
            let stream = async_stream::stream! {
                while let Some(notification) = rx.recv().await {
                    if let NotificationManagerOutput::SpotTradesChanged(trades) = notification {
                        yield trades;
                    }
                }
            };
            Ok(Box::pin(stream))
        } else {
            Err(sqlx::Error::RowNotFound)
        }
    }
}
