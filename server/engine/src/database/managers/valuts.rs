use sqlx::{postgres::PgQueryResult, Postgres, Transaction};
use uuid::Uuid;

use crate::{database::Valut, types::Fraction};

#[derive(Debug)]
pub struct ValutsManager {}

impl ValutsManager {
    pub async fn get<'t, 'p>(
        pool: &'t mut Transaction<'p, Postgres>,
        user_id: Uuid,
        asset_id: Uuid,
    ) -> sqlx::Result<Option<Valut>> {
        sqlx::query_as!(
            Valut,
            r#"
            SELECT
                id,
                balance as "balance: Fraction"
            FROM spot.valuts
            WHERE user_id = $1
            AND asset_id = $2
            "#,
            user_id,
            asset_id
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn create<'t, 'p>(
        pool: &'t mut Transaction<'p, Postgres>,
        user_id: Uuid,
        asset_id: Uuid,
    ) -> sqlx::Result<Valut> {
        sqlx::query_as!(
            Valut,
            r#"
            INSERT INTO spot.valuts
                (user_id, asset_id, balance)
            VALUES ($1, $2, (0,1))
            RETURNING id, balance as "balance: Fraction"
            "#,
            user_id,
            asset_id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn update<'t, 'p>(
        pool: &'t mut Transaction<'p, Postgres>,
        valut: Valut,
    ) -> sqlx::Result<PgQueryResult> {
        sqlx::query_as!(
            Valut,
            r#"
            UPDATE 
                spot.valuts 
            SET
                balance = $2
            WHERE
                id = $1
            "#,
            valut.id,
            valut.balance.to_string() as _
        )
        .execute(pool)
        .await
    }

    pub async fn get_or_create<'t, 'p>(
        pool: &'t mut Transaction<'p, Postgres>,
        user_id: Uuid,
        asset_id: Uuid,
    ) -> sqlx::Result<Valut> {
        Ok(
            if let Some(valut) = Self::get(pool, user_id, asset_id).await? {
                valut
            } else {
                Self::create(pool, user_id, asset_id).await?
            },
        )
    }
}
