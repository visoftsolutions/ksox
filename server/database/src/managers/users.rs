use std::pin::Pin;

use chrono::{DateTime, Utc};
use futures::Stream;
use sqlx::{postgres::PgPool, types::Uuid, Result};

use crate::{
    projections::user::User,
    traits::{manager::Manager, ordered_manager::OrderedManager},
    types::EvmAddress,
};

#[derive(Debug, Clone)]
pub struct UsersManager {
    database: PgPool,
}

impl UsersManager {
    pub fn new(database: PgPool) -> Self {
        UsersManager { database }
    }

    pub async fn get_by_evm_address(&self, evm_address: EvmAddress) -> Result<User> {
        let evm_address_string = evm_address.to_string();
        sqlx::query_as!(
            User,
            r#"
            SELECT
                users.id,
                users.created_at,
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
                RETURNING id, created_at, address as "address: EvmAddress"
            "#,
            evm_address_string.as_str()
        )
        .fetch_one(&self.database)
        .await
    }
}

impl Manager<User> for UsersManager {
    fn get_all(&self) -> Pin<Box<dyn Stream<Item = Result<User>> + Send + '_>> {
        sqlx::query_as!(
            User,
            r#"
            SELECT
                users.id,
                users.created_at,
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
}

impl OrderedManager<User, DateTime<Utc>> for UsersManager {
    fn get_ordered_asc_less(
        &self,
        less_then: DateTime<Utc>,
    ) -> Pin<Box<dyn Stream<Item = Result<User>> + Send + '_>> {
        sqlx::query_as!(
            User,
            r#"
            SELECT
                id,
                created_at,
                address as "address: EvmAddress"
            FROM users
            WHERE created_at <= $1
            ORDER BY created_at ASC
            "#,
            less_then
        )
        .fetch(&self.database)
    }

    fn get_ordered_desc_less(
        &self,
        less_then: DateTime<Utc>,
    ) -> Pin<Box<dyn Stream<Item = Result<User>> + Send + '_>> {
        sqlx::query_as!(
            User,
            r#"
            SELECT
                users.id,
                users.created_at,
                users.address as "address: EvmAddress"
            FROM users
            WHERE created_at <= $1
            ORDER BY created_at DESC
            "#,
            less_then
        )
        .fetch(&self.database)
    }

    fn get_ordered_asc_higher(
        &self,
        higher_then: DateTime<Utc>,
    ) -> Pin<Box<dyn Stream<Item = Result<User>> + Send + '_>> {
        sqlx::query_as!(
            User,
            r#"
            SELECT
                id,
                created_at,
                address as "address: EvmAddress"
            FROM users
            WHERE created_at > $1
            ORDER BY created_at ASC
            "#,
            higher_then
        )
        .fetch(&self.database)
    }

    fn get_ordered_desc_higher(
        &self,
        higher_then: DateTime<Utc>,
    ) -> Pin<Box<dyn Stream<Item = Result<User>> + Send + '_>> {
        sqlx::query_as!(
            User,
            r#"
            SELECT
                id,
                created_at,
                address as "address: EvmAddress"
            FROM users
            WHERE created_at > $1
            ORDER BY created_at DESC
            "#,
            higher_then
        )
        .fetch(&self.database)
    }
}
