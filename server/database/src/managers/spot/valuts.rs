use std::pin::Pin;

use bigdecimal::BigDecimal;
use futures::Stream;
use sqlx::{
    postgres::{PgPool, PgQueryResult},
    types::Uuid,
    Result,
};

use crate::{projections::spot::valut::Valut, traits::manager::Manager, types::Volume};

#[derive(Debug, Clone)]
pub struct ValutsManager {
    database: PgPool,
}

impl ValutsManager {
    pub fn new(database: PgPool) -> Self {
        ValutsManager { database }
    }

    pub async fn get_by_user_id(
        &self,
        id: Uuid,
    ) -> Pin<Box<dyn Stream<Item = Result<Valut>> + Send + '_>> {
        sqlx::query_as!(
            Valut,
            r#"
            SELECT
                id,
                user_id,
                asset_id,
                balance as "balance: Volume"
            FROM spot.valuts
            WHERE user_id = $1
            "#,
            id
        )
        .fetch(&self.database)
    }
}

impl Manager<Valut> for ValutsManager {
    fn get_all(&self) -> Pin<Box<dyn Stream<Item = Result<Valut>> + Send + '_>> {
        sqlx::query_as!(
            Valut,
            r#"
            SELECT
                id,
                user_id,
                asset_id,
                balance as "balance: Volume"
            FROM spot.valuts
            "#
        )
        .fetch(&self.database)
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Valut> {
        sqlx::query_as!(
            Valut,
            r#"
            SELECT
                id,
                user_id,
                asset_id,
                balance as "balance: Volume"
            FROM spot.valuts
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.database)
        .await
    }

    async fn insert(&self, element: Valut) -> Result<PgQueryResult> {
        let balance: BigDecimal = element.balance.into();
        sqlx::query!(
            r#"
            INSERT INTO
                spot.valuts
                (id, user_id, asset_id, balance)
            VALUES
                ($1, $2, $3, $4)
            "#,
            element.id,
            element.user_id,
            element.asset_id,
            balance
        )
        .execute(&self.database)
        .await
    }

    async fn update(&self, element: Valut) -> Result<PgQueryResult> {
        let balance: BigDecimal = element.balance.into();
        sqlx::query!(
            r#"
            UPDATE 
                spot.valuts 
            SET
                user_id = $2,
                asset_id = $3,
                balance = $4
            WHERE
                id = $1
            "#,
            element.id,
            element.user_id,
            element.asset_id,
            balance
        )
        .execute(&self.database)
        .await
    }

    async fn delete(&self, element: Valut) -> Result<PgQueryResult> {
        sqlx::query!(
            r#"
            DELETE FROM 
                spot.valuts 
            WHERE
                id = $1
            "#,
            element.id,
        )
        .execute(&self.database)
        .await
    }
}
