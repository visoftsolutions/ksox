use std::pin::Pin;

use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use futures::{Stream, StreamExt};
use sqlx::{
    postgres::{PgPool, PgQueryResult},
    types::Uuid,
    Result,
};

use crate::{
    managers::notifications::{
        NotificationManagerOutput, NotificationManagerPredicateInput, NotificationManagerSubscriber,
    },
    projections::spot::order::Order,
    traits::{get_modified::GetModified, table_manager::TableManager},
    types::{price_level::PriceLevelOption, Fraction, PriceLevel, SubscribeStream, Volume},
};

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
    ) -> Pin<Box<dyn Stream<Item = Result<Order>> + Send + '_>> {
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
                quote_asset_volume as "quote_asset_volume: Volume",
                base_asset_volume as "base_asset_volume: Volume",
                quote_asset_volume_left as "quote_asset_volume_left: Volume",
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

    pub fn get_for_user_after_last_modification_at(
        &self,
        user_id: Uuid,
        last_modification_at: DateTime<Utc>,
        limit: i64,
        offset: i64,
    ) -> Pin<Box<dyn Stream<Item = Result<Order>> + Send + '_>> {
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
                quote_asset_volume as "quote_asset_volume: Volume",
                base_asset_volume as "base_asset_volume: Volume",
                quote_asset_volume_left as "quote_asset_volume_left: Volume",
                maker_fee as "maker_fee: Fraction"
            FROM spot.orders
            WHERE user_id = $1 AND last_modification_at > $2
            ORDER BY last_modification_at DESC, created_at DESC
            LIMIT $3
            OFFSET $4
            "#,
            user_id,
            last_modification_at,
            limit,
            offset
        )
        .fetch(&self.database)
    }

    pub fn get_active_for_user(
        &self,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Pin<Box<dyn Stream<Item = Result<Order>> + Send + '_>> {
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
                quote_asset_volume as "quote_asset_volume: Volume",
                base_asset_volume as "base_asset_volume: Volume",
                quote_asset_volume_left as "quote_asset_volume_left: Volume",
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

    pub fn get_active_for_user_after_last_modification_at(
        &self,
        user_id: Uuid,
        last_modification_at: DateTime<Utc>,
        limit: i64,
        offset: i64,
    ) -> Pin<Box<dyn Stream<Item = Result<Order>> + Send + '_>> {
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
                quote_asset_volume as "quote_asset_volume: Volume",
                base_asset_volume as "base_asset_volume: Volume",
                quote_asset_volume_left as "quote_asset_volume_left: Volume",
                maker_fee as "maker_fee: Fraction"
            FROM spot.orders
            WHERE user_id = $1 AND is_active = true AND last_modification_at > $2
            ORDER BY last_modification_at DESC, created_at DESC
            LIMIT $3
            OFFSET $4
            "#,
            user_id,
            last_modification_at,
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
    ) -> Pin<Box<dyn Stream<Item = Result<Order>> + Send + '_>> {
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
                quote_asset_volume as "quote_asset_volume: Volume",
                base_asset_volume as "base_asset_volume: Volume",
                quote_asset_volume_left as "quote_asset_volume_left: Volume",
                maker_fee as "maker_fee: Fraction"
            FROM spot.orders
            WHERE quote_asset_id = $1 AND quote_asset_id = $2
            ORDER BY last_modification_at DESC, created_at DESC
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

    pub fn get_orderbook(
        &self,
        quote_asset_id: Uuid,
        base_asset_id: Uuid,
        round: i32,
        limit: i64,
    ) -> Pin<Box<dyn Stream<Item = Result<PriceLevelOption>> + Send + '_>> {
        sqlx::query_as!(
            PriceLevelOption,
            r#"
            SELECT
                ROUND(CAST(base_asset_volume/quote_asset_volume AS NUMERIC), CAST($3 AS INTEGER))::float as price,
                SUM(quote_asset_volume_left) as "volume: Volume"
            FROM spot.orders
            WHERE quote_asset_id = $1
            AND base_asset_id = $2
            AND is_active = true
            GROUP BY price 
            ORDER BY price ASC
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
    ) -> Pin<Box<dyn Stream<Item = Result<PriceLevelOption>> + Send + '_>> {
        sqlx::query_as!(
            PriceLevelOption,
            r#"
            SELECT
                ROUND(CAST(quote_asset_volume/base_asset_volume AS NUMERIC), CAST($3 AS INTEGER))::float as price,
                SUM(quote_asset_volume_left) as "volume: Volume"
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

    pub fn get_ordered_asc_less(
        &self,
        base_asset_id: Uuid,
        quote_asset_id: Uuid,
        base_asset_volume: Volume,
        quote_asset_volume: Volume,
    ) -> Pin<Box<dyn Stream<Item = Result<Order>> + Send + '_>> {
        let quote_asset_volume: BigDecimal = quote_asset_volume.into();
        let base_asset_volume: BigDecimal = base_asset_volume.into();
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
                quote_asset_volume as "quote_asset_volume: Volume",
                base_asset_volume as "base_asset_volume: Volume",
                quote_asset_volume_left as "quote_asset_volume_left: Volume",
                maker_fee as "maker_fee: Fraction"
            FROM spot.orders
            WHERE quote_asset_id = $1
            AND base_asset_id = $2
            AND spot.orders.is_active = true
            AND $3 * CEIL(base_asset_volume*quote_asset_volume_left/quote_asset_volume) <= quote_asset_volume_left * $4
            ORDER BY (base_asset_volume / quote_asset_volume) ASC
            "#,
            quote_asset_id,
            base_asset_id,
            quote_asset_volume,
            base_asset_volume
        )
        .fetch(&self.database)
    }

    pub fn get_ordered_desc_less(
        &self,
        base_asset_id: Uuid,
        quote_asset_id: Uuid,
        base_asset_volume: Volume,
        quote_asset_volume: Volume,
    ) -> Pin<Box<dyn Stream<Item = Result<Order>> + Send + '_>> {
        let quote_asset_volume: BigDecimal = quote_asset_volume.into();
        let base_asset_volume: BigDecimal = base_asset_volume.into();
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
                quote_asset_volume as "quote_asset_volume: Volume",
                base_asset_volume as "base_asset_volume: Volume",
                quote_asset_volume_left as "quote_asset_volume_left: Volume",
                maker_fee as "maker_fee: Fraction"
            FROM spot.orders
            WHERE quote_asset_id = $1
            AND base_asset_id = $2
            AND spot.orders.is_active = true
            AND $3 * CEIL(base_asset_volume*quote_asset_volume_left/quote_asset_volume) <= quote_asset_volume_left * $4
            ORDER BY (base_asset_volume / quote_asset_volume) DESC
            "#,
            quote_asset_id,
            base_asset_id,
            quote_asset_volume,
            base_asset_volume
        )
        .fetch(&self.database)
    }
}

impl TableManager<Order> for OrdersManager {
    fn get_all(&self) -> Pin<Box<dyn Stream<Item = Result<Order>> + Send + '_>> {
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
                quote_asset_volume as "quote_asset_volume: Volume",
                base_asset_volume as "base_asset_volume: Volume",
                quote_asset_volume_left as "quote_asset_volume_left: Volume",
                maker_fee as "maker_fee: Fraction"
            FROM spot.orders
            "#
        )
        .fetch(&self.database)
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Order> {
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
                quote_asset_volume as "quote_asset_volume: Volume",
                base_asset_volume as "base_asset_volume: Volume",
                quote_asset_volume_left as "quote_asset_volume_left: Volume",
                maker_fee as "maker_fee: Fraction"
            FROM spot.orders
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.database)
        .await
    }

    async fn insert(&self, element: Order) -> Result<PgQueryResult> {
        let quote_asset_volume: BigDecimal = element.quote_asset_volume.into();
        let base_asset_volume: BigDecimal = element.base_asset_volume.into();
        let quote_asset_volume_left: BigDecimal = element.quote_asset_volume_left.into();
        let mut transaction = self.database.begin().await?;
        sqlx::query!(
            r#"
            LOCK TABLE spot.valuts IN ACCESS EXCLUSIVE MODE;
            "#
        )
        .execute(&mut transaction)
        .await?;
        let result = sqlx::query!(
            r#"
            INSERT INTO
                spot.orders
                (
                    id, 
                    created_at,
                    user_id, 
                    is_active, 
                    quote_asset_id, 
                    base_asset_id, 
                    quote_asset_volume, 
                    base_asset_volume, 
                    quote_asset_volume_left, 
                    maker_fee
                )
            VALUES
                ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10::fraction)
            "#,
            element.id,
            element.created_at,
            element.user_id,
            element.is_active,
            element.quote_asset_id,
            element.base_asset_id,
            quote_asset_volume,
            base_asset_volume,
            quote_asset_volume_left,
            element.maker_fee.to_string() as _
        )
        .execute(&mut transaction)
        .await?;
        transaction.commit().await?;
        Ok(result)
    }

    async fn update(&self, element: Order) -> Result<PgQueryResult> {
        let quote_asset_volume: BigDecimal = element.quote_asset_volume.into();
        let base_asset_volume: BigDecimal = element.base_asset_volume.into();
        let quote_asset_volume_left: BigDecimal = element.quote_asset_volume_left.into();
        let mut transaction = self.database.begin().await?;
        sqlx::query!(
            r#"
            LOCK TABLE spot.valuts IN ACCESS EXCLUSIVE MODE;
            "#
        )
        .execute(&mut transaction)
        .await?;
        let result = sqlx::query!(
            r#"
            UPDATE 
                spot.orders 
            SET
                created_at = $2,
                user_id = $3,
                is_active = $4,
                quote_asset_id = $5,
                base_asset_id = $6,
                quote_asset_volume = $7,
                base_asset_volume = $8,
                quote_asset_volume_left = $9,
                maker_fee = $10::fraction
            WHERE
                id = $1
            "#,
            element.id,
            element.created_at,
            element.user_id,
            element.is_active,
            element.quote_asset_id,
            element.base_asset_id,
            quote_asset_volume,
            base_asset_volume,
            quote_asset_volume_left,
            element.maker_fee.to_string() as _
        )
        .execute(&self.database)
        .await?;
        transaction.commit().await?;
        Ok(result)
    }

    async fn delete(&self, element: Order) -> Result<PgQueryResult> {
        let mut transaction = self.database.begin().await?;
        sqlx::query!(
            r#"
            LOCK TABLE spot.valuts IN ACCESS EXCLUSIVE MODE;
            "#
        )
        .execute(&mut transaction)
        .await?;
        let result = sqlx::query!(
            r#"
            DELETE FROM 
                spot.orders 
            WHERE
                id = $1
            "#,
            element.id,
        )
        .execute(&self.database)
        .await?;
        transaction.commit().await?;
        Ok(result)
    }
}

impl GetModified<Order> for OrdersManager {
    async fn get_modified(&self, last_modification_at: DateTime<Utc>) -> Result<Vec<Order>> {
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
                quote_asset_volume as "quote_asset_volume: Volume",
                base_asset_volume as "base_asset_volume: Volume",
                quote_asset_volume_left as "quote_asset_volume_left: Volume",
                maker_fee as "maker_fee: Fraction"
            FROM spot.orders
            WHERE last_modification_at > $1
            "#,
            last_modification_at
        )
        .fetch_all(&self.database)
        .await
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

    pub async fn subscribe_active_to_user_id(
        &self,
        user_id: Uuid,
    ) -> Result<SubscribeStream<Vec<Order>>> {
        let p = predicates::function::function(move |input: &NotificationManagerPredicateInput| {
            match input {
                NotificationManagerPredicateInput::SpotOrdersChanged(order) => {
                    order.user_id == user_id && order.is_active
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

    pub async fn subscribe_to_user_id(&self, user_id: Uuid) -> Result<SubscribeStream<Vec<Order>>> {
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
    ) -> Result<SubscribeStream<Vec<PriceLevel>>> {
        let p = predicates::function::function(move |input: &NotificationManagerPredicateInput| {
            match input {
                NotificationManagerPredicateInput::SpotOrdersChanged(order) => {
                    order.quote_asset_id == quote_asset_id && order.base_asset_id == base_asset_id
                },
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
                            .filter_map(|x| async move {
                                if let Ok(x) = x {
                                    if x.price.is_some() && x.volume.is_some() {
                                        Some(PriceLevel {
                                            price: x.price.unwrap(),
                                            volume: x.volume.unwrap(),
                                        })
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            });

                        yield price_levels.collect::<Vec<PriceLevel>>().await;
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
    ) -> Result<SubscribeStream<Vec<PriceLevel>>> {
        let p = predicates::function::function(move |input: &NotificationManagerPredicateInput| {
            match input {
                NotificationManagerPredicateInput::SpotOrdersChanged(order) => {
                    order.quote_asset_id == quote_asset_id && order.base_asset_id == base_asset_id
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
                            .filter_map(|x| async move {
                                if let Ok(x) = x {
                                    if x.price.is_some() && x.volume.is_some() {
                                        Some(PriceLevel {
                                            price: x.price.unwrap(),
                                            volume: x.volume.unwrap(),
                                        })
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            });

                        yield price_levels.collect::<Vec<PriceLevel>>().await;
                    }
                }
            };
            Ok(Box::pin(stream))
        } else {
            Err(sqlx::Error::RowNotFound)
        }
    }
}
