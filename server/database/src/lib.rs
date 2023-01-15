#![allow(dead_code)]
mod managers;
mod projections;

#[cfg(test)]
mod tests {
    use crate::managers::types::EvmAddress;
    use crate::projections::user::User;
    use futures::{Stream, StreamExt};
    use sqlx::{PgPool, Result};
    use std::pin::Pin;

    #[tokio::test]
    async fn address_parse() {
        let db = PgPool::connect(std::env!("DATABASE_URL")).await.unwrap();

        let mut query: Pin<Box<dyn Stream<Item = Result<User>> + Send + '_>> = sqlx::query_as!(
            User,
            r#"SELECT users.id, users.created_at, users.address as "address: EvmAddress" FROM users"#
        ).fetch(&db);

        println!("query result: {:?}", query.next().await);
    }
}
