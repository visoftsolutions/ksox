use chrono::Utc;
use evm::address::Address;
use fraction::Fraction;
use sqlx::{Postgres, Transaction};

use crate::database::projections::withdraw::{Withdraw, WithdrawInsert};

#[derive(Debug, Clone)]
pub struct WithdrawsManager {}

impl WithdrawsManager {
    pub async fn insert<'t, 'p>(
        pool: &'t mut Transaction<'p, Postgres>,
        withdraw: &WithdrawInsert,
    ) -> sqlx::Result<Withdraw> {
        let now = Utc::now();
        sqlx::query_as!(
            Withdraw,
            r#"
            INSERT INTO withdraws
                (created_at, last_modification_at, owner, spender, asset, amount, nonce, deadline, is_active)
            VALUES
                ($1, $2, $3, $4, $5, $6::fraction, $7, $8, $9)
            RETURNING 
                id,
                created_at,
                last_modification_at,
                owner as "owner: Address",
                spender as "spender: Address",
                asset as "asset: Address",
                amount as "amount: Fraction",
                nonce,
                deadline
            "#,
            now,
            now,
            withdraw.owner.to_string() as _,
            withdraw.spender.to_string() as _,
            withdraw.asset.to_string() as _,
            withdraw.amount.to_tuple_string() as _,
            withdraw.nonce,
            withdraw.deadline,
            withdraw.is_active,
        )
        .fetch_one(pool)
        .await
    }
}
