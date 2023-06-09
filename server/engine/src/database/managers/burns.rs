use sqlx::{postgres::PgQueryResult, Postgres, Transaction};
use uuid::Uuid;

use crate::database::projections::burn::{Burn, BurnGet};

#[derive(Debug, Clone)]
pub struct BurnsManager {}

impl BurnsManager {
    pub async fn get_last_for_user<'t, 'p>(
        pool: &'t mut Transaction<'p, Postgres>,
        user_id: Uuid,
    ) -> sqlx::Result<Option<BurnGet>> {
        sqlx::query_as!(
            BurnGet,
            r#"
            SELECT
                last_modification_at
            FROM burns
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
        element: Burn,
    ) -> sqlx::Result<PgQueryResult> {
        sqlx::query!(
            r#"
            INSERT INTO 
                burns (user_id, asset_id, amount)
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
