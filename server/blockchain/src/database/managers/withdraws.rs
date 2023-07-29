use chrono::Utc;
use evm::address::Address;
use evm::txhash::TxHash;
use fraction::Fraction;
use sqlx::{postgres::PgQueryResult, Postgres, Transaction};

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
                (created_at, last_modification_at, maker_address, taker_address, asset_address, tx_hash, amount, confirmations)
            VALUES
                ($1, $2, $3, $4, $5, $6, $7::fraction, $8::fraction)
            RETURNING id, created_at, last_modification_at, maker_address as "maker_address: Address", taker_address as "taker_address: Address", asset_address as "asset_address: Address", tx_hash as "tx_hash: TxHash", amount as "amount: Fraction", confirmations as "confirmations: Fraction"
            "#,
            now,
            now,
            withdraw.maker_address.to_string() as _,
            withdraw.taker_address.to_string() as _,
            withdraw.asset_address.to_string() as _,
            withdraw.tx_hash.to_string() as _,
            withdraw.amount.to_tuple_string() as _,
            withdraw.confirmations.to_tuple_string() as _,
        )
        .fetch_one(pool)
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
