use futures::Stream;
use sqlx::{postgres::PgPool, types::Uuid, Result};
use std::pin::Pin;

use crate::{managers::types::EvmAddress, projections::user::User};

pub struct UsersManager {
    database: PgPool,
}

impl UsersManager {
    pub fn new(database: PgPool) -> Self {
        UsersManager { database }
    }

    pub async fn get_all(&self) -> Pin<Box<dyn Stream<Item = Result<User>> + Send + '_>> {
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

    pub async fn get_by_id(
        &self,
        id: Uuid,
    ) -> Pin<Box<dyn Stream<Item = Result<User>> + Send + '_>> {
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
        .fetch(&self.database)
    }

    pub async fn get_by_evm_address(
        &self,
        evm_address: EvmAddress,
    ) -> Pin<Box<dyn Stream<Item = Result<User>> + Send + '_>> {
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
        .fetch(&self.database)
    }
}
