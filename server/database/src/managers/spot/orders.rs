use std::pin::Pin;

use bigdecimal::BigDecimal;
use futures::{FutureExt, Stream, StreamExt};
use sqlx::{
    postgres::{PgListener, PgPool, PgQueryResult},
    types::Uuid,
    Result,
};

use crate::{
    projections::spot::order::{Order, Status},
    traits::table_manager::TableManager,
    types::{NotifyTrigger, SubscribeStream, Volume},
};

#[derive(Debug, Clone)]
pub struct OrdersManager {
    database: PgPool,
}

impl OrdersManager {
    pub fn new(database: PgPool) -> Self {
        OrdersManager { database }
    }

    pub async fn get_by_user_id(
        &self,
        id: Uuid,
    ) -> Pin<Box<dyn Stream<Item = Result<Order>> + Send + '_>> {
        sqlx::query_as!(
            Order,
            r#"
            SELECT
                id,
                created_at,
                user_id,
                status as "status: Status",
                quote_asset_id,
                base_asset_id,
                quote_asset_volume as "quote_asset_volume: Volume",
                base_asset_volume as "base_asset_volume: Volume"
            FROM spot.orders
            WHERE user_id = $1
            "#,
            id
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
                status as "status: Status",
                quote_asset_id,
                base_asset_id,
                quote_asset_volume as "quote_asset_volume: Volume",
                base_asset_volume as "base_asset_volume: Volume"
            FROM spot.orders
            WHERE quote_asset_id = $1
            AND base_asset_id = $2
            AND spot.orders.status IN ('active', 'partially_filled')
            AND $3 * base_asset_volume <= quote_asset_volume * $4
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
                status as "status: Status",
                quote_asset_id,
                base_asset_id,
                quote_asset_volume as "quote_asset_volume: Volume",
                base_asset_volume as "base_asset_volume: Volume"
            FROM spot.orders
            WHERE quote_asset_id = $1
            AND base_asset_id = $2
            AND spot.orders.status IN ('active', 'partially_filled')
            AND $3 * base_asset_volume <= quote_asset_volume * $4
            ORDER BY (base_asset_volume / quote_asset_volume) DESC
            "#,
            quote_asset_id,
            base_asset_id,
            quote_asset_volume,
            base_asset_volume
        )
        .fetch(&self.database)
    }

    pub async fn create_notify_trigger(&self, id: Uuid) -> Result<NotifyTrigger> {
        sqlx::query!(
            r#"
            SELECT create_orders_notify_trigger($1)
            "#,
            id
        )
        .execute(&self.database)
        .await?;

        let db = self.database.clone();
        let fut = async move {
            sqlx::query!(
                r#"
                SELECT drop_orders_notify_trigger($1)
                "#,
                id
            )
            .execute(&db)
            .await
        };

        Ok(NotifyTrigger::new(
            format!("orders_notify_channel_id_{id}"),
            fut.boxed(),
        ))
    }

    pub async fn get_and_subscribe(&self, user_id: Uuid) -> Result<SubscribeStream<Order>> {
        let mut listener = PgListener::connect_with(&self.database).await?;
        let notify_trigger = self.create_notify_trigger(user_id).await?;
        listener.listen(&notify_trigger.channel_name).await?;

        let subscribe_stream = listener.into_stream().map(|element| {
            element.and_then(|val| {
                serde_json::from_str::<Order>(val.payload())
                    .map_err(|err| sqlx::Error::from(std::io::Error::from(err)))
            })
        });

        let fetch_stream = sqlx::query_as!(
            Order,
            r#"
            SELECT
                id,
                created_at,
                user_id,
                status as "status: Status",
                quote_asset_id,
                base_asset_id,
                quote_asset_volume as "quote_asset_volume: Volume",
                base_asset_volume as "base_asset_volume: Volume"
            FROM spot.orders
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch(&self.database);

        Ok(SubscribeStream::new(
            notify_trigger,
            Box::pin(fetch_stream.chain(subscribe_stream)),
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
                status as "status: Status",
                quote_asset_id,
                base_asset_id,
                quote_asset_volume as "quote_asset_volume: Volume",
                base_asset_volume as "base_asset_volume: Volume"
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
                status as "status: Status",
                quote_asset_id,
                base_asset_id,
                quote_asset_volume as "quote_asset_volume: Volume",
                base_asset_volume as "base_asset_volume: Volume"
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
        sqlx::query!(
            r#"
            INSERT INTO
                spot.orders
                (id, created_at, user_id, status, quote_asset_id, base_asset_id, quote_asset_volume, base_asset_volume)
            VALUES
                ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            element.id,
            element.created_at,
            element.user_id,
            element.status as Status,
            element.quote_asset_id,
            element.base_asset_id,
            quote_asset_volume,
            base_asset_volume
        )
        .execute(&self.database)
        .await
    }

    async fn update(&self, element: Order) -> Result<PgQueryResult> {
        let quote_asset_volume: BigDecimal = element.quote_asset_volume.into();
        let base_asset_volume: BigDecimal = element.base_asset_volume.into();
        sqlx::query!(
            r#"
            UPDATE 
                spot.orders 
            SET
                created_at = $2,
                user_id = $3,
                status = $4,
                quote_asset_id = $5,
                base_asset_id = $6,
                quote_asset_volume = $7,
                base_asset_volume = $8
            WHERE
                id = $1
            "#,
            element.id,
            element.created_at,
            element.user_id,
            element.status as Status,
            element.quote_asset_id,
            element.base_asset_id,
            quote_asset_volume,
            base_asset_volume
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
