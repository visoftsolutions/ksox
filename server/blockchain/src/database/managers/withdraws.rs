use chrono::Utc;
use evm::txhash::TxHash;
use fraction::Fraction;
use sqlx::{postgres::PgQueryResult, PgPool, Postgres, Transaction};

use crate::database::projections::withdraw::{Withdraw, WithdrawInsert};

#[derive(Debug, Clone)]
pub struct WithdrawsManager {
    database: PgPool,
}

impl WithdrawsManager {
    pub fn new(database: PgPool) -> Self {
        Self { database }
    }
}

impl WithdrawsManager {
    pub async fn insert<'t, 'p>(&self, withdraw: WithdrawInsert) -> sqlx::Result<Withdraw> {
        let now = Utc::now();
        sqlx::query_as!(
            Withdraw,
            r#"
            INSERT INTO withdraws
                (created_at, last_modification_at, user_id, asset_id, tx_hash, amount, confirmations)
            VALUES
                ($1, $2, $3, $4, $5, $6::fraction, $7::fraction)
            RETURNING id, created_at, last_modification_at, user_id, asset_id, tx_hash as "tx_hash: TxHash", amount as "amount: Fraction", confirmations as "confirmations: Fraction"
            "#,
            now,
            now,
            withdraw.user_id,
            withdraw.asset_id,
            withdraw.tx_hash.to_string() as _,
            withdraw.amount.to_tuple_string() as _,
            withdraw.confirmations.to_tuple_string() as _,
        )
        .fetch_one(&self.database)
        .await
    }

    pub async fn update<'t, 'p>(
        &self,
        pool: &'t mut Transaction<'p, Postgres>,
        withdraw: Withdraw,
    ) -> sqlx::Result<PgQueryResult> {
        let now = Utc::now();
        sqlx::query!(
            r#"
            UPDATE 
                withdraws
            SET
                confirmations = $2::fraction,
                last_modification_at = $3
            WHERE
                id = $1
            "#,
            withdraw.id,
            withdraw.confirmations.to_tuple_string() as _,
            now
        )
        .execute(pool)
        .await
    }
}