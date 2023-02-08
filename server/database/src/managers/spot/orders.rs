use std::pin::Pin;

use bigdecimal::BigDecimal;
use futures::Stream;
use sqlx::{
    postgres::{PgPool, PgQueryResult},
    types::Uuid,
    Result,
};

use crate::{
    projections::spot::order::{Order, Status},
    traits::manager::Manager,
    types::Volume,
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
}

impl Manager<Order> for OrdersManager {
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
