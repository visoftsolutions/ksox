use std::pin::Pin;

use fraction::Fraction;
use futures::Stream;
use sqlx::{postgres::PgQueryResult, types::chrono::Utc, Postgres, Transaction};
use uuid::Uuid;

use crate::database::projections::order::{Order, OrderGet, OrderInsert, OrderStatus, OrderUpdate};

#[derive(Debug)]
pub struct OrdersManager {}

impl OrdersManager {
    pub async fn get_by_id<'t>(
        t: &'t mut Transaction<'_, Postgres>,
        id: Uuid,
    ) -> sqlx::Result<Option<Order>> {
        sqlx::query_as!(
            Order,
            r#"
            SELECT
                id,
                is_active,
                maker_id,
                quote_asset_id,
                base_asset_id,
                quote_asset_volume_left as "quote_asset_volume_left: Fraction"
            FROM spot.orders
            WHERE spot.orders.id = $1
            "#,
            id
        )
<<<<<<< HEAD
        .fetch_optional(pool.as_mut())
=======
        .fetch_optional(t)
>>>>>>> 0a42fb9 (refactor)
        .await
    }

    pub fn get_orders_with_not_smaller_price<'t>(
        t: &'t mut Transaction<'_, Postgres>,
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
            AND $3::fraction <= price
            ORDER BY price DESC
            "#,
            quote_asset_id,
            base_asset_id,
            price.to_tuple_string() as _
        )
<<<<<<< HEAD
        .fetch(pool.as_mut())
=======
        .fetch(t)
>>>>>>> 0a42fb9 (refactor)
    }

    pub async fn insert<'t>(
        t: &'t mut Transaction<'_, Postgres>,
        element: OrderInsert,
    ) -> sqlx::Result<PgQueryResult> {
        let now = Utc::now();
        sqlx::query!(
            r#"
            INSERT INTO spot.orders
                (maker_id, is_active, quote_asset_id, base_asset_id, price, quote_asset_volume, quote_asset_volume_left, maker_fee, maker_presentation, last_modification_at, created_at)
            VALUES
                ($1, true, $2, $3, $4::fraction, $5::fraction, $6::fraction, $7::fraction, $8, $9, $10)
            "#,
            element.maker_id,
            element.quote_asset_id,
            element.base_asset_id,
            element.price.to_tuple_string() as _,
            element.quote_asset_volume.to_tuple_string() as _,
            element.quote_asset_volume_left.to_tuple_string() as _,
            element.maker_fee.to_tuple_string() as _,
            element.maker_presentation,
            now,
            now
        )
<<<<<<< HEAD
        .execute(pool.as_mut())
=======
        .execute(t)
>>>>>>> 0a42fb9 (refactor)
        .await
    }

    pub async fn update<'t>(
        t: &'t mut Transaction<'_, Postgres>,
        order: OrderUpdate,
    ) -> sqlx::Result<PgQueryResult> {
        sqlx::query!(
            r#"
            UPDATE 
                spot.orders 
            SET
                is_active = $2,
                quote_asset_volume_left = $3::fraction,
                last_modification_at = $4
            WHERE
                id = $1
            "#,
            order.id,
            order.is_active,
            order.quote_asset_volume_left.to_tuple_string() as _,
            Utc::now()
        )
        .execute(t.as_mut())
        .await
    }

    pub async fn update_status<'t>(
        t: &'t mut Transaction<'_, Postgres>,
        order: OrderStatus,
    ) -> sqlx::Result<PgQueryResult> {
        sqlx::query!(
            r#"
            UPDATE 
                spot.orders 
            SET
                is_active = $2,
                last_modification_at = $3
            WHERE
                id = $1
            "#,
            order.id,
            order.is_active,
            Utc::now()
        )
        .execute(t.as_mut())
        .await
    }
}
