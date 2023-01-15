use futures::Stream;
use sqlx::{postgres::PgPool, types::Uuid, Result};
use std::pin::Pin;

use crate::projections::spot::order::{Order, Status};

struct OrdersManager {
    database: PgPool,
}

impl OrdersManager {
    pub fn new(database: PgPool) -> Self {
        OrdersManager { database }
    }

    pub async fn get_all(&self) -> Pin<Box<dyn Stream<Item = Result<Order>> + Send + '_>> {
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

    pub async fn get_by_id(
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
            WHERE id = $1
            "#,
            id
        )
        .fetch(&self.database)
    }
}
