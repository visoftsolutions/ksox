mod managers;
mod projections;

#[cfg(test)]
mod tests {
    use crate::managers::types::EvmAddress;
    use crate::projections::user::User;
    use sqlx::{PgPool, Result, query::Map, Postgres, postgres::PgArguments};

    #[tokio::test]
    async fn address_parse() {
        let db = PgPool::connect(std::env!("DATABASE_URL")).await.unwrap();

        let query: Map<'_, Postgres, _, PgArguments> = sqlx::query_as!(
            User,
            r#"SELECT users.id, users.created_at, users.address as "address: EvmAddress" FROM users"#
        );
        println!("query result: {:?}", query.fetch_all(&db).await);
    }
}
