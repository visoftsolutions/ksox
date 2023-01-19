use std::pin::Pin;

use futures::Stream;
use sqlx::{postgres::PgPool, types::Uuid, Result};

use crate::projections::spot::valut::Valut;

#[derive(Debug, Clone)]
pub struct ValutsManager {
    database: PgPool,
}

impl ValutsManager {
    pub fn new(database: PgPool) -> Self {
        ValutsManager { database }
    }

    pub async fn get_all(&self) -> Pin<Box<dyn Stream<Item = Result<Valut>> + Send + '_>> {
        sqlx::query_as!(
            Valut,
            r#"
            SELECT
                spot.valuts.id,
                spot.valuts.user_id,
                spot.valuts.asset_id,
                spot.valuts.balance
            FROM spot.valuts
            "#
        )
        .fetch(&self.database)
    }

    pub async fn get_by_id(
        &self,
        id: Uuid,
    ) -> Pin<Box<dyn Stream<Item = Result<Valut>> + Send + '_>> {
        sqlx::query_as!(
            Valut,
            r#"
            SELECT
                spot.valuts.id,
                spot.valuts.user_id,
                spot.valuts.asset_id,
                spot.valuts.balance
            FROM spot.valuts
            WHERE id = $1
            "#,
            id
        )
        .fetch(&self.database)
    }

    pub async fn get_by_user_id(
        &self,
        id: Uuid,
    ) -> Pin<Box<dyn Stream<Item = Result<Valut>> + Send + '_>> {
        sqlx::query_as!(
            Valut,
            r#"
            SELECT
                spot.valuts.id,
                spot.valuts.user_id,
                spot.valuts.asset_id,
                spot.valuts.balance
            FROM spot.valuts
            WHERE user_id = $1
            "#,
            id
        )
        .fetch(&self.database)
    }
}
