use chrono::{DateTime, Utc};
use fraction::Fraction;
use sqlx::PgPool;

use crate::database::projections::order::Order;

#[derive(Debug, Clone)]
pub struct OrdersManager {
    database: PgPool,
}
impl OrdersManager {
    pub fn new(database: PgPool) -> Self {
        OrdersManager { database }
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
}
