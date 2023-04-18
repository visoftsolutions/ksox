use std::pin::Pin;

use chrono::{DateTime, Utc};
use fraction::Fraction;
use futures::Stream;
use sqlx::PgPool;
use tokio_stream::StreamExt;
use uuid::Uuid;

use super::notifications::{
    NotificationManagerOutput, NotificationManagerPredicateInput, NotificationManagerSubscriber,
};
use crate::database::projections::order::{Order, PriceLevelOption};

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
                user_id,
                is_active,
                quote_asset_id,
                base_asset_id,
                price as "price: Fraction",
                quote_asset_volume as "quote_asset_volume: Fraction",
                quote_asset_volume_left as "quote_asset_volume_left: Fraction",
                maker_fee as "maker_fee: Fraction"
            FROM spot.orders
            WHERE user_id = $1
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
                user_id,
                is_active,
                quote_asset_id,
                base_asset_id,
                price as "price: Fraction",
                quote_asset_volume as "quote_asset_volume: Fraction",
                quote_asset_volume_left as "quote_asset_volume_left: Fraction",
                maker_fee as "maker_fee: Fraction"
            FROM spot.orders
            WHERE last_modification_at > $1
            "#,
            last_modification_at
        )
        .fetch_all(&self.database)
        .await
    }

    pub fn get_orderbook(
        &self,
        quote_asset_id: Uuid,
        base_asset_id: Uuid,
        round: i32,
        limit: i64,
    ) -> Pin<Box<dyn Stream<Item = sqlx::Result<PriceLevelOption>> + Send + '_>> {
        sqlx::query_as!(
            PriceLevelOption,
            r#"
            SELECT
                ROUND(CAST(price AS NUMERIC), CAST($3 AS INTEGER))::float as price,
                SUM(CAST(quote_asset_volume_left AS NUMERIC))::float as volume
            FROM spot.orders
            WHERE quote_asset_id = $1
            AND base_asset_id = $2
            AND is_active = true
            GROUP BY price 
            ORDER BY price DESC
            LIMIT $4
            "#,
            quote_asset_id,
            base_asset_id,
            round,
            limit
        )
        .fetch(&self.database)
    }

    pub fn get_orderbook_opposite(
        &self,
        quote_asset_id: Uuid,
        base_asset_id: Uuid,
        round: i32,
        limit: i64,
    ) -> Pin<Box<dyn Stream<Item = sqlx::Result<PriceLevelOption>> + Send + '_>> {
        sqlx::query_as!(
            PriceLevelOption,
            r#"
            SELECT
                ROUND(CAST(1 AS NUMERIC) / CAST(price AS NUMERIC), CAST($3 AS INTEGER))::float as price,
                SUM(CAST(quote_asset_volume_left AS NUMERIC))::float as volume
            FROM spot.orders
            WHERE quote_asset_id = $1
            AND base_asset_id = $2
            AND is_active = true
            GROUP BY price
            ORDER BY price ASC
            LIMIT $4
            "#,
            base_asset_id,
            quote_asset_id,
            round,
            limit
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
                user_id,
                is_active,
                quote_asset_id,
                base_asset_id,
                price as "price: Fraction",
                quote_asset_volume as "quote_asset_volume: Fraction",
                quote_asset_volume_left as "quote_asset_volume_left: Fraction",
                maker_fee as "maker_fee: Fraction"
            FROM spot.orders
            WHERE user_id = $1 AND is_active = true
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
                NotificationManagerPredicateInput::SpotOrdersChanged(order) => {
                    order.user_id == user_id
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
                    if let NotificationManagerOutput::SpotOrdersChanged(order) = notification {
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
        precission: i32,
        limit: i64,
    ) -> sqlx::Result<Pin<Box<dyn Stream<Item = Vec<PriceLevelOption>> + Send>>> {
        let p = predicates::function::function(move |input: &NotificationManagerPredicateInput| {
            match input {
                NotificationManagerPredicateInput::SpotOrdersChanged(order) => {
                    (order.quote_asset_id == quote_asset_id && order.base_asset_id == base_asset_id)
                        || (order.quote_asset_id == base_asset_id
                            && order.base_asset_id == quote_asset_id)
                }
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
            let orders_manager = self.orders_manager.clone();
            let stream = async_stream::stream! {
                while let Some(notification) = rx.recv().await {
                    if let NotificationManagerOutput::SpotOrdersChanged(_) = notification {
                        let price_levels = orders_manager
                            .get_orderbook(quote_asset_id, base_asset_id, precission, limit)
                            .map(|x| {
                                if let Ok(price_level) = x {
                                    price_level
                                } else {
                                    PriceLevelOption::default()
                                }
                            });
                        yield price_levels.collect::<Vec<PriceLevelOption>>().await;
                    }
                }
            };
            Ok(Box::pin(stream))
        } else {
            Err(sqlx::Error::RowNotFound)
        }
    }

    pub async fn subscribe_to_orderbook_opposite(
        &self,
        quote_asset_id: Uuid,
        base_asset_id: Uuid,
        precission: i32,
        limit: i64,
    ) -> sqlx::Result<Pin<Box<dyn Stream<Item = Vec<PriceLevelOption>> + Send>>> {
        let p = predicates::function::function(move |input: &NotificationManagerPredicateInput| {
            match input {
                NotificationManagerPredicateInput::SpotOrdersChanged(order) => {
                    (order.quote_asset_id == quote_asset_id && order.base_asset_id == base_asset_id)
                        || (order.quote_asset_id == base_asset_id
                            && order.base_asset_id == quote_asset_id)
                }
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
            let orders_manager = self.orders_manager.clone();
            let stream = async_stream::stream! {
                while let Some(notification) = rx.recv().await {
                    if let NotificationManagerOutput::SpotOrdersChanged(_) = notification {
                        let price_levels = orders_manager
                            .get_orderbook_opposite(quote_asset_id, base_asset_id, precission, limit)
                            .map(|x| {
                                if let Ok(price_level) = x {
                                    price_level
                                } else {
                                    PriceLevelOption::default()
                                }
                            });
                        yield price_levels.collect::<Vec<PriceLevelOption>>().await;
                    }
                }
            };
            Ok(Box::pin(stream))
        } else {
            Err(sqlx::Error::RowNotFound)
        }
    }
}
