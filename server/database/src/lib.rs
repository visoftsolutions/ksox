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
    use futures::StreamExt;
    use sqlx::PgPool;

    use crate::managers::users::UsersManager;

    #[tokio::test]
    async fn basic_users_manager_query() {
        let database = PgPool::connect(std::env::var("DATABASE_URL").unwrap_or_default().as_str())
            .await
            .unwrap();

        let user_manager = UsersManager::new(database);

        let mut query = user_manager.get_all().await;

        let result = query.next().await;

        println!("query result: {:?}", result);
    }
}
