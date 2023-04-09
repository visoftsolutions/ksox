use chrono::{DateTime, Utc};
use sqlx::Result;

pub trait GetModified<T> {
    async fn get_modified(
        &self,
        last_modification_at: DateTime<Utc>,
    ) -> Result<Vec<T>>;
}
