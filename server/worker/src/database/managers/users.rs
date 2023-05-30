use std::pin::Pin;

use sqlx::{postgres::PgPool, Result};
use tokio_stream::Stream;
use uuid::Uuid;

use crate::database::projections::user::{EvmAddress, User};

#[derive(Debug, Clone)]
pub struct UsersManager {
    database: PgPool,
}

impl UsersManager {
    pub fn new(database: PgPool) -> Self {
        UsersManager { database }
    }

    pub async fn get_for_evm_address(&self, evm_address: EvmAddress) -> Result<User> {
        let evm_address_string = evm_address.to_string();
        sqlx::query_as!(
            User,
            r#"
            SELECT
                users.id,
                users.created_at,
                users.last_modification_at,
                users.address as "address: EvmAddress",
                users.name,
                users.phone,
                users.email
            FROM users
            WHERE users.address = $1
            "#,
            evm_address_string.as_str()
        )
        .fetch_one(&self.database)
        .await
    }

    pub async fn insert_with_evmaddress(&self, evm_address: EvmAddress) -> Result<User> {
        let evm_address_string = evm_address.to_string();
        sqlx::query_as!(
            User,
            r#"
            INSERT INTO 
                users
                (last_modification_at, address) VALUES ($1, $2)
                RETURNING id, created_at, last_modification_at, address as "address: EvmAddress", users.name, users.phone, users.email
            "#,
            chrono::Utc::now(),
            evm_address_string.as_str()
        )
        .fetch_one(&self.database)
        .await
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
                users.name,
                users.phone,
                users.email
            FROM users
            WHERE users.last_modification_at > $1
            ORDER BY last_modification_at ASC
            "#,
            last_modification_at
        )
        .fetch_all(&self.database)
        .await
    }

    pub async fn get_by_id(&self, id: Uuid) -> sqlx::Result<User> {
        sqlx::query_as!(
            User,
            r#"
            SELECT
                users.id,
                users.created_at,
                users.last_modification_at,
                users.address as "address: EvmAddress",
                users.name,
                users.phone,
                users.email
            FROM users
            WHERE users.id = $1
            "#,
            id
        )
        .fetch_one(&self.database)
        .await
    }

    pub fn get_all(&self) -> Pin<Box<dyn Stream<Item = sqlx::Result<User>> + Send + '_>> {
        sqlx::query_as!(
            User,
            r#"
            SELECT
                users.id,
                users.created_at,
                users.last_modification_at,
                users.address as "address: EvmAddress",
                users.name,
                users.phone,
                users.email
            FROM users
            "#
        )
        .fetch(&self.database)
    }
}
