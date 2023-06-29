use chrono::{DateTime, Utc};
use evm::address::Address;
use fraction::Fraction;
use sqlx::PgPool;
use uuid::Uuid;

use crate::database::projections::valut::Valut;

#[derive(Debug, Clone)]
pub struct ValutsManager {
    database: PgPool,
}

impl ValutsManager {
    pub fn new(database: PgPool) -> Self {
        ValutsManager { database }
    }

    pub async fn get_modified(
        &self,
        last_modification_at: DateTime<Utc>,
    ) -> sqlx::Result<Vec<Valut>> {
        sqlx::query_as!(
            Valut,
            r#"
            SELECT
                valuts.id,
                valuts.created_at,
                valuts.last_modification_at,
                users.address as "user_address: Address",
                assets.address as "asset_address: Address",
                assets.decimals as "decimals: Fraction",
                valuts.balance as "balance: Fraction"
            FROM valuts
            JOIN users ON valuts.user_id = users.id
            JOIN assets ON valuts.asset_id = assets.id
            WHERE valuts.last_modification_at > $1
            ORDER BY valuts.last_modification_at ASC
            "#,
            last_modification_at,
        )
        .fetch_all(&self.database)
        .await
    }

    pub async fn get_by_id(&self, id: Vec<Uuid>) -> sqlx::Result<Vec<Valut>> {
        sqlx::query_as!(
            Valut,
            r#"
            SELECT
                valuts.id,
                valuts.created_at,
                valuts.last_modification_at,
                users.address as "user_address: Address",
                assets.address as "asset_address: Address",
                assets.decimals as "decimals: Fraction",
                valuts.balance as "balance: Fraction"
            FROM valuts
            JOIN users ON valuts.user_id = users.id
            JOIN assets ON valuts.asset_id = assets.id
            WHERE valuts.id = ANY($1)
            "#,
            &id
        )
        .fetch_all(&self.database)
        .await
    }
}
