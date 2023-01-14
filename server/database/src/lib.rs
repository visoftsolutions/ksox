mod managers;
mod projections;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::managers::types::EvmAddress;
    use crate::projections::user::User;
    use sqlx::PgPool;

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[tokio::test]
    async fn address_parse() {
        let db = PgPool::connect(std::env!("DATABASE_URL")).await.unwrap();

        let query = sqlx::query_as!(
            User,
            r#"SELECT users.id, users.created_at, users.address as "address: EvmAddress" FROM users"#
        ).fetch_all(&db).await;
        println!("query result: {:?}", query);
    }
}
