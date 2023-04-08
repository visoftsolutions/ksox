use std::pin::Pin;

use chrono::{DateTime, Utc};
use futures::Stream;
use sqlx::Result;

pub trait GetModified<T> {
    async fn get_modified(
        &self,
        last_modification_at: DateTime<Utc>,
    ) -> Result<Vec<T>>;
}
