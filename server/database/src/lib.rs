#![allow(dead_code)]
#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]
pub mod managers;
pub mod projections;
pub mod traits;
pub mod types;

pub use sqlx;

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use futures::StreamExt;
    use sqlx::PgPool;
    use uuid::Uuid;

    use crate::{managers::spot::trades::TradesManager, traits::table_manager::TableManager};

    #[tokio::test]
    async fn basic_users_manager_query() {
        let database = PgPool::connect(std::env::var("DATABASE_URL").unwrap_or_default().as_str())
            .await
            .unwrap();

        let trades_manager = TradesManager::new(database);
        let user_id = Uuid::from_str("ead19fc2-9652-444d-8d3c-5256ae80a210").unwrap();
        println!("{user_id}");
        let notify_trigger = trades_manager.create_notify_trigger(user_id).await.unwrap();
        notify_trigger.destroy().await.unwrap();
    }
}
