use sqlx::{postgres::PgQueryResult, Postgres, Transaction};

use crate::database::Trade;

#[derive(Debug)]
pub struct TradesManager {}

impl TradesManager {
    pub async fn insert<'t, 'p>(
        pool: &'t mut Transaction<'p, Postgres>,
        element: Trade,
    ) -> sqlx::Result<PgQueryResult> {
        sqlx::query!(
            r#"
            INSERT INTO spot.trades
                (taker_id, order_id, taker_quote_volume, maker_quote_volume, taker_base_volume, maker_base_volume)
            VALUES
                ($1, $2, $3::fraction, $4::fraction, $5::fraction, $6::fraction)
            "#,
            element.taker_id,
            element.order_id,
            element.taker_quote_volume.to_string() as _,
            element.maker_quote_volume.to_string() as _,
            element.taker_base_volume.to_string() as _,
            element.maker_base_volume.to_string() as _
        )
        .execute(pool)
        .await
    }
}
