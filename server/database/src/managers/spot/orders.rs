use std::pin::Pin;

use bigdecimal::BigDecimal;
use bytes::Bytes;
use futures::{Stream, StreamExt};
use sqlx::{
    postgres::{PgListener, PgPool, PgQueryResult},
    types::Uuid,
    Result,
};
use tokio::{sync::oneshot, task};

use crate::{
    projections::spot::order::Order,
    traits::table_manager::TableManager,
    types::{
        price_level::PriceLevelOption, Fraction, NotifyTrigger, PriceLevel, SubscribeStream, Volume,
    },
    utils::trigger_name,
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
            ORDER BY created_at
            LIMIT $2
            OFFSET $3
            "#,
            user_id,
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
            ORDER BY created_at
            LIMIT $2
            OFFSET $3
            "#,
            user_id,
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
            ORDER BY created_at
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
            ORDER BY price
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

    pub async fn create_notify_trigger_for_user(&self, user_id: Uuid) -> Result<NotifyTrigger> {
        let trigger_name = trigger_name(
            "spot_orders_notify_trigger_for_user",
            vec![Bytes::from(user_id.as_bytes().to_owned().to_vec())],
        );
        sqlx::query!(
            r#"
            SELECT create_spot_orders_notify_trigger_for_user($1, $2)
            "#,
            trigger_name,
            user_id
        )
        .execute(&self.database)
        .await?;

        let db = self.database.clone();
        let trigger_name_clone = trigger_name.clone();
        let (tx, rx) = oneshot::channel::<()>();
        task::spawn(async move {
            if (rx.await).is_err() {
                tracing::error!("drop_signal failed");
            }
            if let Err(err) = sqlx::query!(
                r#"
                SELECT drop_spot_orders_notify_trigger_for_user($1)
                "#,
                trigger_name_clone
            )
            .execute(&db)
            .await
            {
                tracing::error!("{err}");
            }
        });

        Ok(NotifyTrigger::new(format!("c_{trigger_name}"), tx))
    }

    pub async fn subscribe_for_user(&self, user_id: Uuid) -> Result<SubscribeStream<Order>> {
        let mut listener = PgListener::connect_with(&self.database).await?;
        let notify_trigger = self.create_notify_trigger_for_user(user_id).await?;
        listener.listen(&notify_trigger.channel_name).await?;

        let subscribe_stream = listener.into_stream().map(|element| {
            element.and_then(|val| {
                serde_json::from_str::<Order>(val.payload())
                    .map_err(|err| sqlx::Error::from(std::io::Error::from(err)))
            })
        });

        Ok(SubscribeStream::new(
            notify_trigger,
            Box::pin(subscribe_stream),
        ))
    }

    pub async fn create_notify_trigger_active_for_user(
        &self,
        user_id: Uuid,
    ) -> Result<NotifyTrigger> {
        let trigger_name = trigger_name(
            "spot_orders_notify_trigger_active_for_user",
            vec![Bytes::from(user_id.as_bytes().to_owned().to_vec())],
        );
        sqlx::query!(
            r#"
            SELECT create_spot_orders_notify_trigger_active_for_user($1, $2)
            "#,
            trigger_name,
            user_id
        )
        .execute(&self.database)
        .await?;

        let db = self.database.clone();
        let trigger_name_clone = trigger_name.clone();
        let (tx, rx) = oneshot::channel::<()>();
        task::spawn(async move {
            if (rx.await).is_err() {
                tracing::error!("drop_signal failed");
            }
            if let Err(err) = sqlx::query!(
                r#"
                SELECT drop_spot_orders_notify_trigger_active_for_user($1)
                "#,
                trigger_name_clone
            )
            .execute(&db)
            .await
            {
                tracing::error!("{err}");
            }
        });

        Ok(NotifyTrigger::new(format!("c_{trigger_name}"), tx))
    }

    pub async fn subscribe_active_for_user(&self, user_id: Uuid) -> Result<SubscribeStream<Order>> {
        let mut listener = PgListener::connect_with(&self.database).await?;
        let notify_trigger = self.create_notify_trigger_for_user(user_id).await?;
        listener.listen(&notify_trigger.channel_name).await?;

        let subscribe_stream = listener.into_stream().map(|element| {
            element.and_then(|val| {
                serde_json::from_str::<Order>(val.payload())
                    .map_err(|err| sqlx::Error::from(std::io::Error::from(err)))
            })
        });

        Ok(SubscribeStream::new(
            notify_trigger,
            Box::pin(subscribe_stream),
        ))
    }

    pub async fn create_notify_trigger_for_asset_pair(
        &self,
        quote_asset_id: Uuid,
        base_asset_id: Uuid,
    ) -> Result<NotifyTrigger> {
        let trigger_name = trigger_name(
            "spot_orders_notify_trigger_for_asset_pair",
            vec![
                Bytes::from(quote_asset_id.as_bytes().to_owned().to_vec()),
                Bytes::from(base_asset_id.as_bytes().to_owned().to_vec()),
            ],
        );
        sqlx::query!(
            r#"
            SELECT create_spot_orders_notify_trigger_for_asset_pair($1, $2, $3)
            "#,
            trigger_name,
            quote_asset_id,
            base_asset_id
        )
        .execute(&self.database)
        .await?;

        let db = self.database.clone();
        let trigger_name_clone = trigger_name.clone();
        let (tx, rx) = oneshot::channel::<()>();
        task::spawn(async move {
            if (rx.await).is_err() {
                tracing::error!("drop_signal failed");
            }
            if let Err(err) = sqlx::query!(
                r#"
                SELECT drop_spot_orders_notify_trigger_for_asset_pair($1)
                "#,
                trigger_name_clone
            )
            .execute(&db)
            .await
            {
                tracing::error!("{err}");
            }
        });

        Ok(NotifyTrigger::new(format!("c_{trigger_name}"), tx))
    }

    pub async fn subscribe_for_asset_pair(
        &self,
        quote_asset_id: Uuid,
        base_asset_id: Uuid,
    ) -> Result<SubscribeStream<Order>> {
        let mut listener = PgListener::connect_with(&self.database).await?;
        let notify_trigger = self
            .create_notify_trigger_for_asset_pair(quote_asset_id, base_asset_id)
            .await?;
        listener.listen(&notify_trigger.channel_name).await?;

        let subscribe_stream = listener.into_stream().map(|element| {
            element.and_then(|val| {
                serde_json::from_str::<Order>(val.payload())
                    .map_err(|err| sqlx::Error::from(std::io::Error::from(err)))
            })
        });

        Ok(SubscribeStream::new(
            notify_trigger,
            Box::pin(subscribe_stream),
        ))
    }

    pub async fn create_notify_trigger_for_orderbook(
        &self,
        quote_asset_id: Uuid,
        base_asset_id: Uuid,
        precission: i32,
        limit: i64,
    ) -> Result<NotifyTrigger> {
        let trigger_name = trigger_name(
            "spot_orders_notify_trigger_for_orderbook",
            vec![
                Bytes::from(quote_asset_id.as_bytes().to_owned().to_vec()),
                Bytes::from(base_asset_id.as_bytes().to_owned().to_vec()),
                Bytes::from(precission.to_le_bytes().to_owned().to_vec()),
                Bytes::from(limit.to_le_bytes().to_owned().to_vec()),
            ],
        );
        sqlx::query!(
            r#"
            SELECT create_spot_orders_notify_trigger_for_orderbook($1, $2, $3, $4, $5)
            "#,
            trigger_name,
            quote_asset_id,
            base_asset_id,
            precission,
            limit
        )
        .execute(&self.database)
        .await?;

        let db = self.database.clone();
        let trigger_name_clone = trigger_name.clone();
        let (tx, rx) = oneshot::channel::<()>();
        task::spawn(async move {
            if (rx.await).is_err() {
                tracing::error!("drop_signal failed");
            }
            if let Err(err) = sqlx::query!(
                r#"
                SELECT drop_spot_orders_notify_trigger_for_orderbook($1)
                "#,
                trigger_name_clone
            )
            .execute(&db)
            .await
            {
                tracing::error!("{err}");
            }
        });

        Ok(NotifyTrigger::new(format!("c_{trigger_name}"), tx))
    }

    pub async fn subscribe_for_orderbook(
        &self,
        quote_asset_id: Uuid,
        base_asset_id: Uuid,
        precission: i32,
        limit: i64,
    ) -> Result<SubscribeStream<Vec<PriceLevel>>> {
        let mut listener = PgListener::connect_with(&self.database).await?;
        let notify_trigger = self
            .create_notify_trigger_for_orderbook(quote_asset_id, base_asset_id, precission, limit)
            .await?;
        listener.listen(&notify_trigger.channel_name).await?;

        let subscribe_stream = listener.into_stream().map(|element| {
            element.and_then(|val| {
                serde_json::from_str::<Vec<PriceLevel>>(val.payload())
                    .map_err(|err| sqlx::Error::from(std::io::Error::from(err)))
            })
        });

        Ok(SubscribeStream::new(
            notify_trigger,
            Box::pin(subscribe_stream),
        ))
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
        sqlx::query!(
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
        .execute(&self.database)
        .await
    }

    async fn update(&self, element: Order) -> Result<PgQueryResult> {
        let quote_asset_volume: BigDecimal = element.quote_asset_volume.into();
        let base_asset_volume: BigDecimal = element.base_asset_volume.into();
        let quote_asset_volume_left: BigDecimal = element.quote_asset_volume_left.into();
        sqlx::query!(
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
                maker_fee = $10
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
        .await
    }

    async fn delete(&self, element: Order) -> Result<PgQueryResult> {
        sqlx::query!(
            r#"
            DELETE FROM 
                spot.orders 
            WHERE
                id = $1
            "#,
            element.id,
        )
        .execute(&self.database)
        .await
    }
}
