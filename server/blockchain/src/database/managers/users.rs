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

    pub async fn get_by_address(&self, address: Address) -> Result<User> {
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
        .fetch_one(&self.database)
        .await
    }
}
