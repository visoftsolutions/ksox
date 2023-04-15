use sqlx::{postgres::PgQueryResult, PgPool};
use uuid::Uuid;

use crate::{database::Valut, types::Fraction};

#[derive(Debug, Clone)]
pub struct ValutsManager {
    database: PgPool,
}

impl ValutsManager {
    pub fn new(database: PgPool) -> Self {
        Self { database }
    }

    pub async fn get(&self, user_id: Uuid, asset_id: Uuid) -> sqlx::Result<Option<Valut>> {
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
        .fetch_optional(&self.database)
        .await
    }

    pub async fn create(&self, user_id: Uuid, asset_id: Uuid) -> sqlx::Result<Valut> {
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
        .fetch_one(&self.database)
        .await
    }

    pub async fn update(&self, valut: Valut) -> sqlx::Result<PgQueryResult> {
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
        .execute(&self.database)
        .await
    }

    pub async fn get_or_create(&self, user_id: Uuid, asset_id: Uuid) -> sqlx::Result<Valut> {
        Ok(if let Some(valut) = self.get(user_id, asset_id).await? {
            valut
        } else {
            self.create(user_id, asset_id).await?
        })
    }
}
