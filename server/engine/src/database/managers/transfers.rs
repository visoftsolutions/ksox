use sqlx::{postgres::PgQueryResult, types::chrono::Utc, Postgres, Transaction};

use crate::database::projections::transfer::Transfer;

#[derive(Debug)]
pub struct TransfersManager {}

impl TransfersManager {
    pub async fn insert<'t, 'p>(
        pool: &'t mut Transaction<'p, Postgres>,
        element: Transfer,
    ) -> sqlx::Result<PgQueryResult> {
        let now = Utc::now();
        sqlx::query!(
            r#"
            INSERT INTO transfers
                (created_at, last_modification_at, maker_id, taker_id, asset_id, amount)
            VALUES
                ($1, $2, $3, $4, $5, $6::fraction)
            "#,
            now,
            now,
            element.maker_id,
            element.taker_id,
            element.asset_id,
            element.amount.to_tuple_string() as _,
        )
        .execute(pool)
        .await
    }
}
