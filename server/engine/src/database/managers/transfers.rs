use fraction::Fraction;
use sqlx::{postgres::PgQueryResult, types::chrono::Utc, Postgres, Transaction};
use uuid::Uuid;

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

    pub async fn get_by_id<'t, 'p>(
        pool: &'t mut Transaction<'p, Postgres>,
        id: Uuid,
    ) -> sqlx::Result<Transfer> {
        sqlx::query_as!(
            Transfer,
            r#"
            SELECT 
                maker_id,
                taker_id,
                asset_id,
                amount as "amount: Fraction"
            FROM transfers
            WHERE id = $1
            "#,
            id,
        )
        .fetch_one(pool)
        .await
    }
}
