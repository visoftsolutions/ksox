use chrono::{DateTime, Utc};
use evm::address::Address;
use fraction::Fraction;
use num_traits::Zero;
use sqlx::{postgres::PgQueryResult, PgPool, Postgres, Transaction};
use uuid::Uuid;
use value::Value;

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
                valuts.balance as "balance: Value"
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
                valuts.balance as "balance: Value"
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

    pub async fn get<'t>(
        t: &'t mut Transaction<'_, Postgres>,
        user_id: Uuid,
        asset_id: Uuid,
    ) -> sqlx::Result<Option<Valut>> {
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
                valuts.balance as "balance: Value"
            FROM valuts
            JOIN users ON valuts.user_id = users.id
            JOIN assets ON valuts.asset_id = assets.id
            WHERE user_id = $1
            AND asset_id = $2
            "#,
            user_id,
            asset_id
        )
        .fetch_optional(t.as_mut())
        .await
    }

    pub async fn create<'t>(
        t: &'t mut Transaction<'_, Postgres>,
        user_id: Uuid,
        asset_id: Uuid,
    ) -> sqlx::Result<PgQueryResult> {
        let now = Utc::now();
        let value = Value::Finite(Fraction::zero());
        sqlx::query!(
            r#"
            INSERT INTO valuts
                (user_id, asset_id, balance, last_modification_at, created_at)
            VALUES ($1, $2, $3::text, $4, $5)
            "#,
            user_id,
            asset_id,
            serde_json::to_string(&value).unwrap_or_default(),
            now,
            now
        )
        .execute(t.as_mut())
        .await
    }

    pub async fn get_or_create<'t>(
        t: &'t mut Transaction<'_, Postgres>,
        user_id: Uuid,
        asset_id: Uuid,
    ) -> sqlx::Result<Valut> {
        Ok(
            if let Some(valut) = Self::get(t, user_id, asset_id).await? {
                valut
            } else {
                Self::create(t, user_id, asset_id).await?;
                Self::get(t, user_id, asset_id)
                    .await?
                    .ok_or(sqlx::Error::RowNotFound)?
            },
        )
    }
}
