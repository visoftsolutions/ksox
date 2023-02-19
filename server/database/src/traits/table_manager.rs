use std::pin::Pin;

use futures::Stream;
use sqlx::{postgres::PgQueryResult, types::Uuid, Result};

pub trait TableManager<T> {
    fn get_all(&self) -> Pin<Box<dyn Stream<Item = Result<T>> + Send + '_>>;
    async fn get_by_id(&self, id: Uuid) -> Result<T>;
    async fn insert(&self, element: T) -> Result<PgQueryResult>;
    async fn update(&self, element: T) -> Result<PgQueryResult>;
    async fn delete(&self, element: T) -> Result<PgQueryResult>;
}
