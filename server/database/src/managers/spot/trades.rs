use std::pin::Pin;

use futures::Stream;
use sqlx::{postgres::PgPool, types::Uuid, Result};

use crate::projections::spot::trade::Trade;

pub struct TradesManager {
    database: PgPool,
}

impl TradesManager {
    pub fn new(database: PgPool) -> Self {
        TradesManager { database }
    }

    pub async fn get_all(&self) -> Pin<Box<dyn Stream<Item = Result<Trade>> + Send + '_>> {
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
            "#
        )
        .fetch(&self.database)
    }

    pub async fn get_by_id(
        &self,
        id: Uuid,
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
            WHERE id = $1
            "#,
            id
        )
        .fetch(&self.database)
    }
}
