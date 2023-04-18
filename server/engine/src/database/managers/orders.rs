use std::pin::Pin;

use fraction::Fraction;
use futures::Stream;
use sqlx::{postgres::PgQueryResult, Postgres, Transaction};
use uuid::Uuid;

use crate::database::{Order, OrderGet, OrderInsert, OrderStatus, OrderUpdate};

#[derive(Debug)]
pub struct OrdersManager {}

impl OrdersManager {
    pub async fn get_by_id<'t, 'p>(
        pool: &'t mut Transaction<'p, Postgres>,
        id: Uuid,
    ) -> sqlx::Result<Option<Order>> {
        sqlx::query_as!(
            Order,
            r#"
            SELECT
                id,
                is_active,
                user_id,
                quote_asset_id,
                base_asset_id,
                quote_asset_volume_left as "quote_asset_volume_left: Fraction"
            FROM spot.orders
            WHERE spot.orders.id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await
    }

    pub fn get_orders_with_smaller_price<'t, 'p>(
        pool: &'t mut Transaction<'p, Postgres>,
        quote_asset_id: Uuid,
        base_asset_id: Uuid,
        price: Fraction,
    ) -> Pin<Box<dyn Stream<Item = sqlx::Result<OrderGet>> + Send + 't>> {
        sqlx::query_as!(
            OrderGet,
            r#"
            SELECT
                id,
                price as "price: Fraction",
                quote_asset_volume_left as "quote_asset_volume_left: Fraction",
                maker_fee as "maker_fee: Fraction"
            FROM spot.orders
            WHERE quote_asset_id = $1
            AND base_asset_id = $2
            AND is_active = true
            AND quote_asset_volume_left > (0,1)::fraction
            AND $3::fraction >= price
            ORDER BY price ASC
            "#,
            quote_asset_id,
            base_asset_id,
            price as _
        )
        .fetch(pool)
    }

    pub async fn insert<'t, 'p>(
        pool: &'t mut Transaction<'p, Postgres>,
        element: OrderInsert,
    ) -> sqlx::Result<PgQueryResult> {
        sqlx::query!(
            r#"
            INSERT INTO spot.orders
                (user_id, is_active, quote_asset_id, base_asset_id, price, quote_asset_volume, quote_asset_volume_left, maker_fee)
            VALUES
                ($1, true, $2, $3, $4::fraction, $5::fraction, $6::fraction, $7::fraction)
            "#,
            element.user_id,
            element.quote_asset_id,
            element.base_asset_id,
            element.price.to_string() as _,
            element.quote_asset_volume.to_string() as _,
            element.quote_asset_volume_left.to_string() as _,
            element.maker_fee.to_string() as _
        )
        .execute(pool)
        .await
    }

    pub async fn update<'t, 'p>(
        pool: &'t mut Transaction<'p, Postgres>,
        order: OrderUpdate,
    ) -> sqlx::Result<PgQueryResult> {
        sqlx::query!(
            r#"
            UPDATE 
                spot.orders 
            SET
                is_active = $2,
                quote_asset_volume_left = $3::fraction
            WHERE
                id = $1
            "#,
            order.id,
            order.is_active,
            order.quote_asset_volume_left.to_string() as _
        )
        .execute(pool)
        .await
    }

    pub async fn update_status<'t, 'p>(
        pool: &'t mut Transaction<'p, Postgres>,
        order: OrderStatus,
    ) -> sqlx::Result<PgQueryResult> {
        sqlx::query!(
            r#"
            UPDATE 
                spot.orders 
            SET
                is_active = $2
            WHERE
                id = $1
            "#,
            order.id,
            order.is_active,
        )
        .execute(pool)
        .await
    }
}
