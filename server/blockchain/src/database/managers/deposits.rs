use crate::database::projections::deposit::{Deposit, DepositInsert};
use chrono::Utc;
use evm::address::Address;
use evm::txhash::TxHash;
use fraction::Fraction;
use sqlx::{postgres::PgQueryResult, Postgres, Transaction};

#[derive(Debug, Clone)]
pub struct DepositsManager {}

impl DepositsManager {
    pub async fn insert<'t, 'p>(
        pool: &'t mut Transaction<'p, Postgres>,
        deposit: &DepositInsert,
    ) -> sqlx::Result<Deposit> {
        let now = Utc::now();
        sqlx::query_as!(
            Deposit,
            r#"
            INSERT INTO deposits
            (created_at, last_modification_at, maker_address, taker_address, asset_address, tx_hash, amount, confirmations)
            VALUES
                ($1, $2, $3, $4, $5, $6, $7::fraction, $8::fraction)
            RETURNING id, created_at, last_modification_at, maker_address as "maker_address: Address", taker_address as "taker_address: Address", asset_address as "asset_address: Address", tx_hash as "tx_hash: TxHash", amount as "amount: Fraction", confirmations as "confirmations: Fraction"
            "#,
            now,
            now,
            deposit.maker_address.to_string() as _,
            deposit.taker_address.to_string() as _,
            deposit.asset_address.to_string() as _,
            deposit.tx_hash.to_string() as _,
            deposit.amount.to_tuple_string() as _,
            deposit.confirmations.to_tuple_string() as _,
        )
        .fetch_one(pool)
        .await
    }

    pub async fn update<'t, 'p>(
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
