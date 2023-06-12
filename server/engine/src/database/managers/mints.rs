use sqlx::{postgres::PgQueryResult, Postgres, Transaction};
use uuid::Uuid;

use crate::database::projections::mint::{Mint, MintGet};

#[derive(Debug, Clone)]
pub struct MintsManager {}

impl MintsManager {
    pub async fn get_last_for_user<'t, 'p>(
        pool: &'t mut Transaction<'p, Postgres>,
        user_id: Uuid,
    ) -> sqlx::Result<Option<MintGet>> {
        sqlx::query_as!(
            MintGet,
            r#"
            SELECT
                last_modification_at
            FROM mints
            WHERE user_id = $1
            ORDER BY last_modification_at DESC
            LIMIT 1 
            "#,
            user_id
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn insert<'t, 'p>(
        pool: &'t mut Transaction<'p, Postgres>,
        element: Mint,
    ) -> sqlx::Result<PgQueryResult> {
        sqlx::query!(
            r#"
            INSERT INTO 
                mints (user_id, asset_id, amount)
            VALUES
                ($1, $2, $3::fraction)
            "#,
            element.user_id,
            element.asset_id,
            element.amount.to_tuple_string() as _
        )
        .execute(pool)
        .await
    }
}
