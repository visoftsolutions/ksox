use chrono::Utc;
use evm::txhash::TxHash;
use fraction::Fraction;
use sqlx::{postgres::PgQueryResult, Postgres, Transaction};

use super::FlowManager;
use crate::database::projections::{Flow, FlowInsert};
#[derive(Debug, Clone)]
pub struct DepositsManager {}

impl DepositsManager {
    pub fn new() -> Self {
        Self {}
    }
}

impl FlowManager for DepositsManager {
    async fn insert<'t, 'p>(
        &self,
        pool: &'t mut Transaction<'p, Postgres>,
        flow: FlowInsert,
    ) -> sqlx::Result<Flow> {
        let now = Utc::now();
        sqlx::query_as!(
            Flow,
            r#"
            INSERT INTO deposits
                (created_at, last_modification_at, user_id, asset_id, tx_hash, amount, confirmations)
            VALUES
                ($1, $2, $3, $4, $5, $6::fraction, $7::fraction)
            RETURNING id, created_at, last_modification_at, user_id, asset_id, tx_hash as "tx_hash: TxHash", amount as "amount: Fraction", confirmations as "confirmations: Fraction"
            "#,
            now,
            now,
            flow.user_id,
            flow.asset_id,
            flow.tx_hash.to_string() as _,
            flow.amount.to_tuple_string() as _,
            flow.confirmations.to_tuple_string() as _,
        )
        .fetch_one(pool)
        .await
    }

    async fn update<'t, 'p>(
        &self,
        pool: &'t mut Transaction<'p, Postgres>,
        flow: Flow,
    ) -> sqlx::Result<PgQueryResult> {
        let now = Utc::now();
        sqlx::query!(
            r#"
            UPDATE 
                deposits
            SET
                confirmations = $2::fraction,
                last_modification_at = $3
            WHERE
                id = $1
            "#,
            flow.id,
            flow.confirmations.to_tuple_string() as _,
            now
        )
        .execute(pool)
        .await
    }
}
