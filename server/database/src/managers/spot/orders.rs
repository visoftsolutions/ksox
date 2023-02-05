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
    traits::{manager::Manager, ordered_manager::OrderedManager},
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
                quote_asset_volume,
                base_asset_volume
            FROM spot.orders
            WHERE user_id = $1
            "#,
            id
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
                quote_asset_volume,
                base_asset_volume
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
                quote_asset_volume,
                base_asset_volume
            FROM spot.orders
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.database)
        .await
    }

    async fn insert(&self, element: Order) -> Result<PgQueryResult> {
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
            element.quote_asset_volume,
            element.base_asset_volume,
        )
        .execute(&self.database)
        .await
    }

    async fn update(&self, element: Order) -> Result<PgQueryResult> {
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
            element.quote_asset_volume,
            element.base_asset_volume,
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

impl OrderedManager<Order, BigDecimal> for OrdersManager {
    fn get_ordered_asc_less(
        &self,
        less_than: BigDecimal,
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
                quote_asset_volume,
                base_asset_volume
            FROM spot.orders
            WHERE spot.orders.status IN ('active', 'partially_filled') AND (base_asset_volume / quote_asset_volume) <= $1
            ORDER BY (base_asset_volume / quote_asset_volume) ASC
            "#,
            less_than
        )
        .fetch(&self.database)
    }

    fn get_ordered_desc_less(
        &self,
        less_than: BigDecimal,
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
                quote_asset_volume,
                base_asset_volume
            FROM spot.orders
            WHERE spot.orders.status IN ('active', 'partially_filled') AND (base_asset_volume / quote_asset_volume) <= $1
            ORDER BY (base_asset_volume / quote_asset_volume) DESC
            "#,
            less_than
        )
        .fetch(&self.database)
    }

    fn get_ordered_asc_higher(
        &self,
        higher_then: BigDecimal,
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
                quote_asset_volume,
                base_asset_volume
            FROM spot.orders
            WHERE spot.orders.status IN ('active', 'partially_filled') AND (base_asset_volume / quote_asset_volume) > $1
            ORDER BY (base_asset_volume / quote_asset_volume) ASC
            "#,
            higher_then
        )
        .fetch(&self.database)
    }

    fn get_ordered_desc_higher(
        &self,
        higher_then: BigDecimal,
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
                quote_asset_volume,
                base_asset_volume
            FROM spot.orders
            WHERE spot.orders.status IN ('active', 'partially_filled') AND (base_asset_volume / quote_asset_volume) > $1
            ORDER BY (base_asset_volume / quote_asset_volume) DESC
            "#,
            higher_then
        )
        .fetch(&self.database)
    }
}
