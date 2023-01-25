use std::pin::Pin;

use futures::Stream;
use sqlx::{types::Uuid, Result};

pub trait Manager<O> {
    async fn get_all(&self) -> Pin<Box<dyn Stream<Item = Result<O>> + Send + '_>>;
    async fn get_by_id(&self, id: Uuid) -> Pin<Box<dyn Stream<Item = Result<O>> + Send + '_>>;
}
