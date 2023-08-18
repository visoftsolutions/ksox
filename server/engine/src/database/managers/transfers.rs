use crate::database::managers::Id;
use crate::database::projections::transfer::Transfer;
use fraction::Fraction;
use sqlx::{types::chrono::Utc, Postgres, Transaction};
use uuid::Uuid;

#[derive(Debug)]
pub struct TransfersManager {}

impl TransfersManager {
    pub async fn insert<'t>(
        t: &'t mut Transaction<'_, Postgres>,
        element: Transfer,
    ) -> sqlx::Result<Uuid> {
        let now = Utc::now();
        sqlx::query_as!(
            Id,
            r#"
            INSERT INTO transfers (
                created_at,
                last_modification_at,
                from_valut_id,
                to_valut_id,
                fee_harvester_user_id,
                asset_id,
                amount,
                fee
            )
            VALUES
                ($1, $2, $3, $4, $5, $6, $7::fraction, $8::fraction)
            RETURNING id
            "#,
            now,
            now,
            element.from_valut_id,
            element.to_valut_id,
            element.fee_harvester_user_id,
            element.asset_id,
            element.amount.to_tuple_string() as _,
            element.fee.to_tuple_string() as _,
        )
        .fetch_one(t.as_mut())
        .await
        .map(|e| e.id)
    }

    pub async fn get_by_id<'t>(
        t: &'t mut Transaction<'_, Postgres>,
        id: Uuid,
    ) -> sqlx::Result<Transfer> {
        sqlx::query_as!(
            Transfer,
            r#"
            SELECT 
                from_valut_id,
                to_valut_id,
                fee_harvester_user_id,
                asset_id,
                amount as "amount: Fraction",
                fee as "fee: Fraction"
            FROM transfers
            WHERE id = $1
            "#,
            id,
        )
        .fetch_one(t.as_mut())
        .await
    }
}
