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
    use core::time;
    use std::{str::FromStr, thread::sleep};

    use chrono::Utc;
    use futures::StreamExt;
    use num_bigint::BigInt;
    use sqlx::PgPool;
    use uuid::Uuid;

    use crate::{managers::spot::trades::TradesManager};

    #[tokio::test]
    async fn basic_users_manager_query() {
        let database = PgPool::connect(std::env::var("DATABASE_URL").unwrap_or_default().as_str())
            .await
            .unwrap();

        let trades_manager = TradesManager::new(database);
        let user_id = Uuid::from_str("ead19fc2-9652-444d-8d3c-5256ae80a210").unwrap();
        println!("{user_id}");

        let mut stream = trades_manager.get_and_subscribe(Uuid::from_str("ead19fc2-9652-444d-8d3c-5256ae80a210").unwrap()).await.unwrap();
        while let Some(result) = stream.next().await {
            match result {
                Ok(trade) => {
                    println!("{:#?}", trade);
                },
                Err(err) => {
                    println!("{err}");
                }
            }
        }
        println!("NONE");

        sleep(time::Duration::from_secs(5));
        stream.destroy().await.unwrap();
    }
}
