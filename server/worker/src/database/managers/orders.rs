use std::{io, pin::Pin};

use chrono::{DateTime, Utc};
use fraction::{num_traits::Inv, Fraction};
use futures::{Stream, StreamExt, TryStreamExt};
use pricelevel::PriceLevel;
use sqlx::PgPool;
use uuid::Uuid;

use super::notifications::{
    NotificationManagerOutput, NotificationManagerPredicateInput, NotificationManagerSubscriber,
};
use crate::database::projections::order::Order;

#[derive(Debug, Clone)]
pub struct OrdersManager {
    database: PgPool,
}
impl OrdersManager {
    pub fn new(database: PgPool) -> Self {
        OrdersManager { database }
    }

    pub fn get_for_user(
        &self,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Pin<Box<dyn Stream<Item = sqlx::Result<Order>> + Send + '_>> {
        sqlx::query_as!(
            Order,
            r#"
            SELECT
                id,
                created_at,
                last_modification_at,
                maker_id,
                is_active,
                quote_asset_id,
                base_asset_id,
                price as "price: Fraction",
                quote_asset_volume as "quote_asset_volume: Fraction",
                quote_asset_volume_left as "quote_asset_volume_left: Fraction",
                maker_fee as "maker_fee: Fraction",
                maker_presentation
            FROM spot.orders
            WHERE maker_id = $1
            ORDER BY last_modification_at DESC, created_at DESC
            LIMIT $2
            OFFSET $3
            "#,
            user_id,
            limit,
            offset
        )
        .fetch(&self.database)
    }

    pub async fn get_modified(
        &self,
        last_modification_at: DateTime<Utc>,
    ) -> sqlx::Result<Vec<Order>> {
        sqlx::query_as!(
            Order,
            r#"
            SELECT
                id,
                created_at,
                last_modification_at,
                maker_id,
                is_active,
                quote_asset_id,
                base_asset_id,
                price as "price: Fraction",
                quote_asset_volume as "quote_asset_volume: Fraction",
                quote_asset_volume_left as "quote_asset_volume_left: Fraction",
                maker_fee as "maker_fee: Fraction",
                maker_presentation
            FROM spot.orders
            WHERE last_modification_at > $1
            ORDER BY last_modification_at ASC
            "#,
            last_modification_at
        )
        .fetch_all(&self.database)
        .await
    }

    pub fn get_active_orders(
        &self,
        quote_asset_id: Uuid,
        base_asset_id: Uuid,
    ) -> Pin<Box<dyn Stream<Item = sqlx::Result<PriceLevel>> + Send + '_>> {
        sqlx::query_as!(
            PriceLevel,
            r#"
            SELECT
            price as "price: Fraction",
            quote_asset_volume_left as "volume: Fraction"
            FROM spot.orders
            WHERE quote_asset_id = $1
            AND base_asset_id = $2
            AND is_active = true
            ORDER BY price DESC
            "#,
            quote_asset_id,
            base_asset_id
        )
        .fetch(&self.database)
    }

    pub fn get_active_orders_opposite(
        &self,
        quote_asset_id: Uuid,
        base_asset_id: Uuid,
    ) -> Pin<Box<dyn Stream<Item = sqlx::Result<PriceLevel>> + Send + '_>> {
        sqlx::query_as!(
            PriceLevel,
            r#"
            SELECT
            price as "price: Fraction",
            quote_asset_volume_left as "volume: Fraction"
            FROM spot.orders
            WHERE base_asset_id = $1
            AND quote_asset_id = $2
            AND is_active = true
            ORDER BY price ASC
            "#,
            quote_asset_id,
            base_asset_id
        )
        .fetch(&self.database)
    }

    pub fn get_active_for_user(
        &self,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Pin<Box<dyn Stream<Item = sqlx::Result<Order>> + Send + '_>> {
        sqlx::query_as!(
            Order,
            r#"
            SELECT
                id,
                created_at,
                last_modification_at,
                maker_id,
                is_active,
                quote_asset_id,
                base_asset_id,
                price as "price: Fraction",
                quote_asset_volume as "quote_asset_volume: Fraction",
                quote_asset_volume_left as "quote_asset_volume_left: Fraction",
                maker_fee as "maker_fee: Fraction",
                maker_presentation
            FROM spot.orders
            WHERE maker_id = $1 AND is_active = true
            ORDER BY last_modification_at DESC, created_at DESC
            LIMIT $2
            OFFSET $3
            "#,
            user_id,
            limit,
            offset
        )
        .fetch(&self.database)
    }

    pub fn get_orderbook(
        &self,
        quote_asset_id: Uuid,
        base_asset_id: Uuid,
        precision: Fraction,
    ) -> Pin<Box<dyn Stream<Item = io::Result<PriceLevel>> + Send + '_>> {
        PriceLevel::aggregate_with_accuracy(
            self.get_active_orders(quote_asset_id, base_asset_id)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))
                .boxed(),
            precision,
        )
    }

    pub fn get_orderbook_opposite(
        &self,
        quote_asset_id: Uuid,
        base_asset_id: Uuid,
        precision: Fraction,
    ) -> Pin<Box<dyn Stream<Item = io::Result<PriceLevel>> + Send + '_>> {
        PriceLevel::aggregate_with_accuracy(
            self.get_active_orders_opposite(quote_asset_id, base_asset_id)
                .map_ok(|e| e.inv())
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))
                .boxed(),
            precision,
        )
    }
}

#[derive(Debug, Clone)]
pub struct OrdersNotificationManager {
    notification_manager_subscriber: NotificationManagerSubscriber,
    orders_manager: OrdersManager,
}
impl OrdersNotificationManager {
    pub fn new(
        notification_manager_subscriber: NotificationManagerSubscriber,
        orders_manager: OrdersManager,
    ) -> Self {
        Self {
            notification_manager_subscriber,
            orders_manager,
        }
    }

    pub async fn subscribe_to_user_id(
        &self,
        user_id: Uuid,
    ) -> sqlx::Result<Pin<Box<dyn Stream<Item = Vec<Order>> + Send>>> {
        let p = predicates::function::function(move |input: &NotificationManagerPredicateInput| {
            match input {
                NotificationManagerPredicateInput::SpotOrders(order) => order.maker_id == user_id,
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
                    if let NotificationManagerOutput::SpotOrders(order) = notification {
                        yield order;
                    }
                }
            };
            Ok(Box::pin(stream))
        } else {
            Err(sqlx::Error::RowNotFound)
        }
    }

    pub async fn subscribe_to_orderbook(
        &self,
        quote_asset_id: Uuid,
        base_asset_id: Uuid,
        precision: Fraction,
        limit: usize,
    ) -> io::Result<Pin<Box<dyn Stream<Item = io::Result<Vec<PriceLevel>>> + Send + '_>>> {
        let p = predicates::function::function(move |input: &NotificationManagerPredicateInput| {
            match input {
                NotificationManagerPredicateInput::SpotOrders(order) => {
                    (order.quote_asset_id == quote_asset_id && order.base_asset_id == base_asset_id)
                        || (order.quote_asset_id == base_asset_id
                            && order.base_asset_id == quote_asset_id)
                }
                NotificationManagerPredicateInput::SpotTrades(trade) => {
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
            let orders_manager = self.orders_manager.clone();
            let stream = async_stream::stream! {
                while let Some(notification) = rx.recv().await {
                    if let NotificationManagerOutput::SpotOrders(_) = notification {
                        yield orders_manager.get_orderbook(quote_asset_id, base_asset_id, precision.clone()).take(limit).try_collect::<Vec<PriceLevel>>().await;
                    }
                }
            };
            Ok(Box::pin(stream))
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                sqlx::Error::RowNotFound.to_string(),
            ))
        }
    }

    pub async fn subscribe_to_orderbook_opposite(
        &self,
        quote_asset_id: Uuid,
        base_asset_id: Uuid,
        precision: Fraction,
        limit: usize,
    ) -> io::Result<Pin<Box<dyn Stream<Item = io::Result<Vec<PriceLevel>>> + Send + '_>>> {
        let p = predicates::function::function(move |input: &NotificationManagerPredicateInput| {
            match input {
                NotificationManagerPredicateInput::SpotOrders(order) => {
                    (order.quote_asset_id == quote_asset_id && order.base_asset_id == base_asset_id)
                        || (order.quote_asset_id == base_asset_id
                            && order.base_asset_id == quote_asset_id)
                }
                NotificationManagerPredicateInput::SpotTrades(trade) => {
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
            let orders_manager = self.orders_manager.clone();
            let stream = async_stream::stream! {
                while let Some(notification) = rx.recv().await {
                    if let NotificationManagerOutput::SpotOrders(_) = notification {
                        yield orders_manager.get_orderbook_opposite(quote_asset_id, base_asset_id, precision.clone()).take(limit).try_collect::<Vec<PriceLevel>>().await;
                    }
                }
            };
            Ok(Box::pin(stream))
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                sqlx::Error::RowNotFound.to_string(),
            ))
        }
    }
}
