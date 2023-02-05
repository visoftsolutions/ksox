use std::pin::Pin;

use futures::Stream;
use sqlx::{
    postgres::{PgPool, PgQueryResult},
    types::Uuid,
    Result,
};

use crate::{projections::spot::trade::Trade, traits::manager::Manager};

#[derive(Debug, Clone)]
pub struct TradesManager {
    database: PgPool,
}

impl TradesManager {
    pub fn new(database: PgPool) -> Self {
        TradesManager { database }
    }

    fn get_ordered_asc(
        &self,
        user_id: Uuid,
    ) -> Pin<Box<dyn Stream<Item = Result<Trade>> + Send + '_>> {
        sqlx::query_as!(
            Trade,
            r#"
            SELECT
                spot.trades.id,
                spot.trades.created_at,
                spot.trades.taker_id,
                spot.trades.order_id,
                spot.trades.taker_quote_volume,
                spot.trades.taker_base_volume,
                spot.trades.maker_quote_volume,
                spot.trades.maker_base_volume
            FROM spot.trades
            JOIN spot.orders ON spot.trades.order_id = spot.orders.id
            WHERE spot.orders.user_id = $1
            ORDER BY created_at ASC
            "#,
            user_id
        )
        .fetch(&self.database)
    }

    fn get_ordered_desc(
        &self,
        user_id: Uuid,
    ) -> Pin<Box<dyn Stream<Item = Result<Trade>> + Send + '_>> {
        sqlx::query_as!(
            Trade,
            r#"
            SELECT
                spot.trades.id,
                spot.trades.created_at,
                spot.trades.taker_id,
                spot.trades.order_id,
                spot.trades.taker_quote_volume,
                spot.trades.taker_base_volume,
                spot.trades.maker_quote_volume,
                spot.trades.maker_base_volume
            FROM spot.trades
            JOIN spot.orders ON spot.trades.order_id = spot.orders.id
            WHERE spot.orders.user_id = $1
            ORDER BY created_at DESC
            "#,
            user_id
        )
        .fetch(&self.database)
    }
}

impl Manager<Trade> for TradesManager {
    fn get_all(&self) -> Pin<Box<dyn Stream<Item = Result<Trade>> + Send + '_>> {
        sqlx::query_as!(
            Trade,
            r#"
            SELECT
                id,
                created_at,
                taker_id,
                order_id,
                taker_quote_volume,
                taker_base_volume,
                maker_quote_volume,
                maker_base_volume
            FROM spot.trades
            "#
        )
        .fetch(&self.database)
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Trade> {
        sqlx::query_as!(
            Trade,
            r#"
            SELECT
                id,
                created_at,
                taker_id,
                order_id,
                taker_quote_volume,
                taker_base_volume,
                maker_quote_volume,
                maker_base_volume
            FROM spot.trades
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.database)
        .await
    }

    async fn insert(&self, element: Trade) -> Result<PgQueryResult> {
        sqlx::query!(
            r#"
            INSERT INTO
                spot.trades
                (id, created_at, taker_id, order_id, taker_quote_volume, maker_quote_volume, taker_base_volume, maker_base_volume)
            VALUES
                ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            element.id,
            element.created_at,
            element.taker_id,
            element.order_id,
            element.taker_quote_volume,
            element.maker_quote_volume,
            element.taker_base_volume,
            element.maker_base_volume
        )
        .execute(&self.database)
        .await
    }

    async fn update(&self, element: Trade) -> Result<PgQueryResult> {
        sqlx::query!(
            r#"
            UPDATE 
                spot.trades 
            SET
                created_at = $2,
                taker_id = $3,
                order_id = $4,
                taker_quote_volume = $5,
                maker_quote_volume = $6,
                taker_base_volume = $7,
                maker_base_volume = $8
            WHERE
                id = $1
            "#,
            element.id,
            element.created_at,
            element.taker_id,
            element.order_id,
            element.taker_quote_volume,
            element.maker_quote_volume,
            element.taker_base_volume,
            element.maker_base_volume
        )
        .execute(&self.database)
        .await
    }

    async fn delete(&self, element: Trade) -> Result<PgQueryResult> {
        sqlx::query!(
            r#"
            DELETE FROM 
                spot.trades 
            WHERE
                id = $1
            "#,
            element.id,
        )
        .execute(&self.database)
        .await
    }
}
