use fraction::Fraction;
use num_traits::Zero;
use sqlx::{postgres::PgQueryResult, types::chrono::Utc, Postgres, Transaction};
use uuid::Uuid;
use value::Value;

use crate::database::projections::valut::Valut;

#[derive(Debug)]
pub struct ValutsManager {}

impl ValutsManager {
    pub async fn get_by_id<'t>(
        t: &'t mut Transaction<'_, Postgres>,
        id: &Uuid,
    ) -> sqlx::Result<Valut> {
        sqlx::query_as!(
            Valut,
            r#"
            SELECT
                id,
                balance as "balance: Value"
            FROM valuts
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(t.as_mut())
        .await
    }

    pub async fn get<'t>(
        t: &'t mut Transaction<'_, Postgres>,
        user_id: &Uuid,
        asset_id: &Uuid,
    ) -> sqlx::Result<Option<Valut>> {
        sqlx::query_as!(
            Valut,
            r#"
            SELECT
                id,
                balance as "balance: Value"
            FROM valuts
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
        user_id: &Uuid,
        asset_id: &Uuid,
    ) -> sqlx::Result<Valut> {
        let now = Utc::now();
        let value = Value::Finite(Fraction::zero());
        sqlx::query_as!(
            Valut,
            r#"
            INSERT INTO valuts
                (user_id, asset_id, balance, last_modification_at, created_at)
            VALUES ($1, $2, $3::text, $4, $5)
            RETURNING id, balance as "balance: Value"
            "#,
            user_id,
            asset_id,
            serde_json::to_string(&value).unwrap_or_default(),
            now,
            now
        )
        .fetch_one(t.as_mut())
        .await
    }

    pub async fn update<'t>(
        t: &'t mut Transaction<'_, Postgres>,
        valut: Valut,
    ) -> sqlx::Result<PgQueryResult> {
        sqlx::query_as!(
            Valut,
            r#"
            UPDATE 
                valuts 
            SET
                balance = $2,
                last_modification_at = $3
            WHERE
                id = $1
            "#,
            valut.id,
            serde_json::to_string(&valut.balance).unwrap_or_default(),
            Utc::now()
        )
        .execute(t.as_mut())
        .await
    }

    pub async fn get_or_create<'t>(
        t: &'t mut Transaction<'_, Postgres>,
        user_id: &Uuid,
        asset_id: &Uuid,
    ) -> sqlx::Result<Valut> {
        Ok(
            if let Some(valut) = Self::get(t, user_id, asset_id).await? {
                valut
            } else {
                Self::create(t, user_id, asset_id).await?
            },
        )
    }
}
