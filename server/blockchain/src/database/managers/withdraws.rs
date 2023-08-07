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
                (created_at, last_modification_at, maker_address, taker_address, asset_address, amount, deadline)
            VALUES
                ($1, $2, $3, $4, $5, $6::fraction, $7)
            RETURNING id, created_at, last_modification_at, maker_address as "maker_address: Address", taker_address as "taker_address: Address", asset_address as "asset_address: Address", amount as "amount: Fraction", deadline
            "#,
            now,
            now,
            withdraw.maker_address.to_string() as _,
            withdraw.taker_address.to_string() as _,
            withdraw.asset_address.to_string() as _,
            withdraw.amount.to_tuple_string() as _,
            withdraw.deadline,
        )
        .fetch_one(pool)
        .await
    }
}
