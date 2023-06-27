use sqlx::{Transaction, Postgres, postgres::PgQueryResult};

use super::projections::{FlowInsert, Flow};

pub mod deposits;
pub mod withdraws;

pub trait FlowManager {
    async fn insert<'t, 'p>(
        pool: &'t mut Transaction<'p, Postgres>,
        deposit: FlowInsert,
    ) -> sqlx::Result<Flow>;

    async fn update<'t, 'p>(
        pool: &'t mut Transaction<'p, Postgres>,
        deposit: Flow,
    ) -> sqlx::Result<PgQueryResult>;
}