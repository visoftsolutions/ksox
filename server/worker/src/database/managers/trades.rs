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
                spot.trades.id,
                spot.trades.created_at,
                spot.trades.last_modification_at,
                spot.trades.quote_asset_id,
                spot.trades.base_asset_id,
                spot.trades.taker_id,
                spot.trades.taker_presentation,
                spot.orders.maker_id,
                spot.orders.maker_presentation,
                spot.trades.price as "price: Fraction",
                spot.trades.taker_quote_volume as "taker_quote_volume: Fraction",
                spot.trades.taker_base_volume as "taker_base_volume: Fraction",
                spot.trades.maker_quote_volume as "maker_quote_volume: Fraction",
                spot.trades.maker_base_volume as "maker_base_volume: Fraction"
            FROM spot.trades
            JOIN spot.orders ON spot.orders.id = spot.trades.order_id
            WHERE spot.trades.last_modification_at > $1
            ORDER BY spot.trades.last_modification_at ASC
            "#,
            last_modification_at
        )
        .fetch_all(&self.database)
        .await
    }

    pub fn get_for_user_id(
        &self,
        taker_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Pin<Box<dyn Stream<Item = sqlx::Result<Trade>> + Send + '_>> {
        sqlx::query_as!(
            Trade,
            r#"
            SELECT
                spot.trades.id,
                spot.trades.created_at,
                spot.trades.last_modification_at,
                spot.trades.quote_asset_id,
                spot.trades.base_asset_id,
                spot.trades.taker_id,
                spot.trades.taker_presentation,
                spot.orders.maker_id,
                spot.orders.maker_presentation,
                spot.trades.price as "price: Fraction",
                spot.trades.taker_quote_volume as "taker_quote_volume: Fraction",
                spot.trades.taker_base_volume as "taker_base_volume: Fraction",
                spot.trades.maker_quote_volume as "maker_quote_volume: Fraction",
                spot.trades.maker_base_volume as "maker_base_volume: Fraction"
            FROM spot.trades
            JOIN spot.orders ON spot.orders.id = spot.trades.order_id
            WHERE spot.trades.taker_id = $1 OR spot.orders.maker_id = $1
            ORDER BY spot.trades.created_at DESC
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
                spot.trades.id,
                spot.trades.created_at,
                spot.trades.last_modification_at,
                spot.trades.quote_asset_id,
                spot.trades.base_asset_id,
                spot.trades.taker_id,
                spot.trades.taker_presentation,
                spot.orders.maker_id,
                spot.orders.maker_presentation,
                spot.trades.price as "price: Fraction",
                spot.trades.taker_quote_volume as "taker_quote_volume: Fraction",
                spot.trades.taker_base_volume as "taker_base_volume: Fraction",
                spot.trades.maker_quote_volume as "maker_quote_volume: Fraction",
                spot.trades.maker_base_volume as "maker_base_volume: Fraction"
            FROM spot.trades
            JOIN spot.orders ON spot.orders.id = spot.trades.order_id
            WHERE (spot.trades.quote_asset_id = $1 AND spot.trades.base_asset_id = $2) OR (spot.trades.quote_asset_id = $2 AND spot.trades.base_asset_id = $1)
            ORDER BY spot.trades.created_at DESC
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
                spot.trades.id,
                spot.trades.created_at,
                spot.trades.last_modification_at,
                spot.trades.quote_asset_id,
                spot.trades.base_asset_id,
                spot.trades.taker_id,
                spot.trades.taker_presentation,
                spot.orders.maker_id,
                spot.orders.maker_presentation,
                spot.trades.price as "price: Fraction",
                spot.trades.taker_quote_volume as "taker_quote_volume: Fraction",
                spot.trades.taker_base_volume as "taker_base_volume: Fraction",
                spot.trades.maker_quote_volume as "maker_quote_volume: Fraction",
                spot.trades.maker_base_volume as "maker_base_volume: Fraction"
            FROM spot.trades
            JOIN spot.orders ON spot.orders.id = spot.trades.order_id
            WHERE (spot.trades.quote_asset_id = $1 AND spot.trades.base_asset_id = $2) OR (spot.trades.quote_asset_id = $2 AND spot.trades.base_asset_id = $1)
            ORDER BY spot.trades.created_at DESC
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
                spot.trades.id,
                spot.trades.created_at,
                spot.trades.last_modification_at,
                spot.trades.quote_asset_id,
                spot.trades.base_asset_id,
                spot.trades.taker_id,
                spot.trades.taker_presentation,
                spot.orders.maker_id,
                spot.orders.maker_presentation,
                spot.trades.price as "price: Fraction",
                spot.trades.taker_quote_volume as "taker_quote_volume: Fraction",
                spot.trades.taker_base_volume as "taker_base_volume: Fraction",
                spot.trades.maker_quote_volume as "maker_quote_volume: Fraction",
                spot.trades.maker_base_volume as "maker_base_volume: Fraction"
            FROM spot.trades
            JOIN spot.orders ON spot.orders.id = spot.trades.order_id
            WHERE ((spot.trades.quote_asset_id = $1 AND spot.trades.base_asset_id = $2) OR (spot.trades.quote_asset_id = $2 AND spot.trades.base_asset_id = $1)) AND spot.trades.created_at >= $3
            ORDER BY spot.trades.created_at
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
                spot.trades.id,
                spot.trades.created_at,
                spot.trades.last_modification_at,
                spot.trades.quote_asset_id,
                spot.trades.base_asset_id,
                spot.trades.taker_id,
                spot.trades.taker_presentation,
                spot.orders.maker_id,
                spot.orders.maker_presentation,
                spot.trades.price as "price: Fraction",
                spot.trades.taker_quote_volume as "taker_quote_volume: Fraction",
                spot.trades.taker_base_volume as "taker_base_volume: Fraction",
                spot.trades.maker_quote_volume as "maker_quote_volume: Fraction",
                spot.trades.maker_base_volume as "maker_base_volume: Fraction"
            FROM spot.trades
            JOIN spot.orders ON spot.orders.id = spot.trades.order_id
            WHERE ((spot.trades.quote_asset_id = $1 AND spot.trades.base_asset_id = $2) OR (spot.trades.quote_asset_id = $2 AND spot.trades.base_asset_id = $1)) AND spot.trades.created_at >= $3 AND spot.trades.created_at < $4
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

    pub async fn subscribe_to_user(
        &self,
        user_id: Uuid,
    ) -> sqlx::Result<Pin<Box<dyn Stream<Item = Vec<Trade>> + Send>>> {
        let p = predicates::function::function(move |input: &NotificationManagerPredicateInput| {
            match input {
                NotificationManagerPredicateInput::SpotTradesChanged(trade) => {
                    trade.taker_id == user_id || trade.maker_id == user_id
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
