use std::pin::Pin;

use futures::Stream;
use sqlx::{
    postgres::{PgPool, PgQueryResult},
    types::Uuid,
    Result,
};

use crate::{
    projections::spot::order::{Order, Status},
    traits::manager::Manager,
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
                spot.orders.id,
                spot.orders.created_at,
                spot.orders.user_id,
                spot.orders.status as "status: Status",
                spot.orders.quote_asset_id,
                spot.orders.base_asset_id,
                spot.orders.quote_asset_volume,
                spot.orders.base_asset_price
            FROM spot.orders
            WHERE user_id = $1
            "#,
            id
        )
        .fetch(&self.database)
    }
}

impl Manager<Order> for OrdersManager {
    async fn get_all(&self) -> Pin<Box<dyn Stream<Item = Result<Order>> + Send + '_>> {
        sqlx::query_as!(
            Order,
            r#"
            SELECT
                spot.orders.id,
                spot.orders.created_at,
                spot.orders.user_id,
                spot.orders.status as "status: Status",
                spot.orders.quote_asset_id,
                spot.orders.base_asset_id,
                spot.orders.quote_asset_volume,
                spot.orders.base_asset_price
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
                    spot.orders.id,
                    spot.orders.created_at,
                    spot.orders.user_id,
                    spot.orders.status as "status: Status",
                    spot.orders.quote_asset_id,
                    spot.orders.base_asset_id,
                    spot.orders.quote_asset_volume,
                    spot.orders.base_asset_price
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
                (id, created_at, user_id, status, quote_asset_id, base_asset_id, quote_asset_volume, base_asset_price) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            element.id,
            element.created_at,
            element.user_id,
            element.status as Status,
            element.quote_asset_id,
            element.base_asset_id,
            element.quote_asset_volume,
            element.base_asset_price,
        )
        .execute(&self.database)
        .await
    }
}
