use chrono::Utc;
use evm::txhash::TxHash;
use fraction::Fraction;
use sqlx::{postgres::PgQueryResult, PgPool, Postgres, Transaction};

use crate::database::projections::deposit::{Deposit, DepositInsert};

#[derive(Debug, Clone)]
pub struct DepositsManager {
    database: PgPool,
}

impl DepositsManager {
    pub fn new(database: PgPool) -> Self {
        Self { database }
    }
}

impl DepositsManager {
    pub async fn insert(&self, deposit: DepositInsert) -> sqlx::Result<Deposit> {
        let now = Utc::now();
        sqlx::query_as!(
            Deposit,
            r#"
            INSERT INTO deposits
                (created_at, last_modification_at, user_id, asset_id, tx_hash, amount, confirmations)
            VALUES
                ($1, $2, $3, $4, $5, $6::fraction, $7::fraction)
            RETURNING id, created_at, last_modification_at, user_id, asset_id, tx_hash as "tx_hash: TxHash", amount as "amount: Fraction", confirmations as "confirmations: Fraction"
            "#,
            now,
            now,
            deposit.user_id,
            deposit.asset_id,
            deposit.tx_hash.to_string() as _,
            deposit.amount.to_tuple_string() as _,
            deposit.confirmations.to_tuple_string() as _,
        )
        .fetch_one(&self.database)
        .await
    }

    pub async fn update<'t, 'p>(
        &self,
        pool: &'t mut Transaction<'p, Postgres>,
        deposit: Deposit,
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
            deposit.id,
            deposit.confirmations.to_tuple_string() as _,
            now
        )
        .execute(pool)
        .await
    }
}
