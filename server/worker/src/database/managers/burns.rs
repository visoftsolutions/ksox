use fraction::Fraction;
use sqlx::{postgres::PgQueryResult, PgPool};
use uuid::Uuid;

use crate::database::projections::{mint::Mint};

#[derive(Debug, Clone)]
pub struct MintsManager {
    database: PgPool,
}

impl MintsManager {
    pub fn new(database: PgPool) -> Self {
        Self { database }
    }

    pub async fn get_last_for_user(&self, user_id: Uuid) -> sqlx::Result<Option<Mint>> {
        sqlx::query_as!(
            Mint,
            r#"
            SELECT
                id,
                created_at,
                last_modification_at,
                user_id,
                asset_id,
                amount as "amount: Fraction"
            FROM burns
            WHERE user_id = $1
            ORDER BY last_modification_at DESC
            LIMIT 1 
            "#,
            user_id
        )
        .fetch_optional(&self.database)
        .await
    }
}