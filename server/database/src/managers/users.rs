use std::pin::Pin;

use futures::Stream;
use sqlx::{postgres::PgPool, types::Uuid, Result};

use crate::{projections::user::User, traits::{table_manager::TableManager, get_modified::GetModified}, types::EvmAddress};

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
                users.address as "address: EvmAddress"
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
                (address) VALUES ($1)
                RETURNING id, created_at, last_modification_at, address as "address: EvmAddress"
            "#,
            evm_address_string.as_str()
        )
        .fetch_one(&self.database)
        .await
    }
}

impl TableManager<User> for UsersManager {
    fn get_all(&self) -> Pin<Box<dyn Stream<Item = Result<User>> + Send + '_>> {
        sqlx::query_as!(
            User,
            r#"
            SELECT
                users.id,
                users.created_at,
                users.last_modification_at,
                users.address as "address: EvmAddress"
            FROM users
            "#
        )
        .fetch(&self.database)
    }

    async fn get_by_id(&self, id: Uuid) -> Result<User> {
        sqlx::query_as!(
            User,
            r#"
            SELECT
                users.id,
                users.created_at,
                users.last_modification_at,
                users.address as "address: EvmAddress"
            FROM users
            WHERE users.id = $1
            "#,
            id
        )
        .fetch_one(&self.database)
        .await
    }

    async fn insert(&self, element: User) -> Result<sqlx::postgres::PgQueryResult> {
        let address_string = element.address.to_string();
        sqlx::query!(
            r#"
            INSERT INTO
                users
                (id, created_at, address)
            VALUES
                ($1, $2, $3)
            "#,
            element.id,
            element.created_at,
            address_string
        )
        .execute(&self.database)
        .await
    }

    async fn update(&self, element: User) -> Result<sqlx::postgres::PgQueryResult> {
        let address_string = element.address.to_string();
        sqlx::query!(
            r#"
            UPDATE 
                users
            SET
                created_at = $2,
                address = $3
            WHERE
                id = $1
            "#,
            element.id,
            element.created_at,
            address_string
        )
        .execute(&self.database)
        .await
    }

    async fn delete(&self, element: User) -> Result<sqlx::postgres::PgQueryResult> {
        sqlx::query!(
            r#"
            DELETE FROM 
                users
            WHERE
                id = $1
            "#,
            element.id,
        )
        .execute(&self.database)
        .await
    }
}

impl GetModified<User> for UsersManager {
    async fn get_modified(&self, last_modification_at: chrono::DateTime<chrono::Utc>) -> Result<Vec<User>> {
        sqlx::query_as!(
            User,
            r#"
            SELECT
                users.id,
                users.created_at,
                users.last_modification_at,
                users.address as "address: EvmAddress"
            FROM users
            WHERE users.last_modification_at > $1
            "#,
            last_modification_at
        )
        .fetch_all(&self.database)
        .await
    }
}
