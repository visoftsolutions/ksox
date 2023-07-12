 use chrono::Utc;
use evm::address::Address;
use sqlx::{postgres::PgPool, Result};

use crate::database::projections::user::User;

#[derive(Debug, Clone)]
pub struct UsersManager {
    database: PgPool,
}

impl UsersManager {
    pub fn new(database: PgPool) -> Self {
        Self { database }
    }

    async fn insert_with_address(&self, address: Address) -> Result<User> {
        let now = Utc::now();
        sqlx::query_as!(
            User,
            r#"
            INSERT INTO 
                users
                (last_modification_at, address) VALUES ($1, $2)
                RETURNING id
            "#,
            now,
            &address.to_string()
        )
        .fetch_one(&self.database)
        .await
    }

    pub async fn get_by_address(&self, address: Address) -> Result<Option<User>> {
        sqlx::query_as!(
            User,
            r#"
            SELECT
                id
            FROM users
            WHERE address = $1
            "#,
            address.to_string()
        )
        .fetch_optional(&self.database)
        .await
    }

    pub async fn get_or_create_by_address(&self, address: Address) -> Result<User> {
        match self.get_by_address(address.clone()).await? {
            Some(user) => Ok(user),
            None => {
                self.insert_with_address(address).await
            }
        }

    }
}
