use chrono::Utc;
use evm::address::Address;
use sqlx::{Postgres, Result, Transaction};
use uuid::Uuid;

use crate::database::projections::user::User;

#[derive(Debug, Clone)]
pub struct UsersManager {}

impl UsersManager {
    async fn insert_with_address<'t, 'p>(
        pool: &'t mut Transaction<'p, Postgres>,
        address: Address,
    ) -> Result<User> {
        let now = Utc::now();
        sqlx::query_as!(
            User,
            r#"
            INSERT INTO 
                users
                (last_modification_at, address) VALUES ($1, $2)
                RETURNING id, address as "address: Address"
            "#,
            now,
            &address.to_string()
        )
        .fetch_one(pool)
        .await
    }

    pub async fn get_by_address<'t, 'p>(
        pool: &'t mut Transaction<'p, Postgres>,
        address: Address,
    ) -> Result<Option<User>> {
        sqlx::query_as!(
            User,
            r#"
            SELECT
                id,
                address as "address: Address"
            FROM users
            WHERE address = $1
            "#,
            address.to_string()
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn get_or_create_by_address<'t, 'p>(
        pool: &'t mut Transaction<'p, Postgres>,
        address: Address,
    ) -> Result<User> {
        match Self::get_by_address(pool, address.clone()).await? {
            Some(user) => Ok(user),
            None => Self::insert_with_address(pool, address).await,
        }
    }

    pub async fn get_by_id<'t, 'p>(
        pool: &'t mut Transaction<'p, Postgres>,
        id: Uuid,
    ) -> Result<User> {
        sqlx::query_as!(
            User,
            r#"
            SELECT
                id,
                address as "address: Address"
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await
    }
}