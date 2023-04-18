use sqlx::{postgres::PgQueryResult, Postgres, Transaction};

use sqlx::types::chrono::Utc;

use crate::database::projections::trade::Trade;

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
                (quote_asset_id, base_asset_id, taker_id, order_id, price, taker_quote_volume, maker_quote_volume, taker_base_volume, maker_base_volume, last_modification_at)
            VALUES
                ($1, $2, $3, $4, $5::fraction, $6::fraction, $7::fraction, $8::fraction, $9::fraction, $10)
            "#,
            element.quote_asset_id,
            element.base_asset_id,
            element.taker_id,
            element.order_id,
            element.taker_price.to_string() as _,
            element.taker_quote_volume.to_string() as _,
            element.maker_quote_volume.to_string() as _,
            element.taker_base_volume.to_string() as _,
            element.maker_base_volume.to_string() as _,
            Utc::now()
        )
        .execute(pool)
        .await
    }
}
