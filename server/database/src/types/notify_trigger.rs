use std::pin::Pin;

use futures::Future;
use sqlx::{postgres::PgQueryResult, Result};

pub struct NotifyTrigger {
    pub channel_name: String,
    drop_fn: Pin<Box<dyn Future<Output = Result<PgQueryResult>> + Send>>,
}

impl NotifyTrigger {
    pub fn new(
        channel_name: String,
        drop_fn: Pin<Box<dyn Future<Output = Result<PgQueryResult>> + Send>>,
    ) -> Self {
        NotifyTrigger {
            drop_fn,
            channel_name,
        }
    }

    pub async fn destroy(self) -> Result<PgQueryResult> {
        self.drop_fn.await
    }
}
