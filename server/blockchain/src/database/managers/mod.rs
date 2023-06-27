use sqlx::{postgres::PgQueryResult, Postgres, Transaction};

use super::projections::{Flow, FlowInsert};

pub mod deposits;
pub mod withdraws;

pub trait FlowManager {
    async fn insert<'t, 'p>(
        &self,
        pool: &'t mut Transaction<'p, Postgres>,
        flow: FlowInsert,
    ) -> sqlx::Result<Flow>;

    async fn update<'t, 'p>(
        &self,
        pool: &'t mut Transaction<'p, Postgres>,
        flow: Flow,
    ) -> sqlx::Result<PgQueryResult>;
}
