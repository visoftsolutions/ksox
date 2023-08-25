use sqlx::{postgres::PgPool, Result};

use crate::database::projections::user::{EvmAddress, User};

#[derive(Debug, Clone)]
pub struct UsersManager {
    database: PgPool,
}

impl UsersManager {
    pub fn new(database: PgPool) -> Self {
        UsersManager { database }
    }

    pub async fn get_modified(
        &self,
        last_modification_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<Vec<User>> {
        sqlx::query_as!(
            User,
            r#"
            SELECT
                users.id,
                users.created_at,
                users.last_modification_at,
                users.address as "address: EvmAddress",
                users.name
            FROM users
            WHERE users.last_modification_at > $1
            ORDER BY last_modification_at ASC
            "#,
            last_modification_at
        )
        .fetch_all(&self.database)
        .await
    }
}
