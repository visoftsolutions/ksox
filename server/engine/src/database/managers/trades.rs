use sqlx::{postgres::PgQueryResult, types::chrono::Utc, Postgres, Transaction};

use crate::database::projections::trade::Trade;

#[derive(Debug)]
pub struct TradesManager {}

impl TradesManager {
    pub async fn insert<'t>(
        t: &'t mut Transaction<'_, Postgres>,
        element: Trade,
    ) -> sqlx::Result<PgQueryResult> {
        let now = Utc::now();
        sqlx::query!(
            r#"
            INSERT INTO spot.trades
                (quote_asset_id, base_asset_id, taker_id, taker_presentation, order_id, price, taker_quote_volume, maker_quote_volume, taker_quote_volume_transfer_id, maker_quote_volume_transfer_id, last_modification_at, created_at)
            VALUES
                ($1, $2, $3, $4, $5, $6::fraction, $7::fraction, $8::fraction, $9, $10, $11, $12)
            "#,
            element.quote_asset_id,
            element.base_asset_id,
            element.taker_id,
            element.taker_presentation,
            element.order_id,
            element.taker_price.to_tuple_string() as _,
            element.taker_quote_volume.to_tuple_string() as _,
            element.maker_quote_volume.to_tuple_string() as _,
            element.taker_quote_volume_transfer_id,
            element.maker_quote_volume_transfer_id,
            now,
            now
        )
        .execute(t.as_mut())
        .await
    }
}
