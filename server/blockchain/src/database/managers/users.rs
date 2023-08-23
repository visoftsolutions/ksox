use chrono::Utc;
use evm::address::Address;
use sqlx::{Postgres, Result, Transaction};
use uuid::Uuid;

use crate::database::projections::user::User;

#[derive(Debug, Clone)]
pub struct UsersManager {}

impl UsersManager {
    async fn insert_with_address<'t>(
        t: &'t mut Transaction<'_, Postgres>,
        address: &Address,
    ) -> Result<User> {
        let now = Utc::now();
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO "users"."user"
                (created_at, last_modification_at)
            VALUES
                ($1, $2)
            RETURNING id
            "#,
            now,
            now,
        )
        .fetch_one(t.as_mut())
        .await?;
        sqlx::query!(
            r#"
            INSERT INTO "users"."address"
                (network_id, user_id, address)
            VALUES
                ($1, $2)
            RETURNING id
            "#
        );
        user
    }

    pub async fn get_by_address<'t>(
        t: &'t mut Transaction<'_, Postgres>,
        address: &Address,
    ) -> Result<User> {
        sqlx::query_as!(
            User,
            r#"
            SELECT 
                u.id
            FROM "users"."user" u
            JOIN "users"."address" a ON u.id = a.user_id
            WHERE a.address = $1
            "#,
            address.to_string()
        )
        .fetch_one(t.as_mut())
        .await
    }

    pub async fn get_or_create_by_address<'t>(
        t: &'t mut Transaction<'_, Postgres>,
        address: &Address,
    ) -> Result<User> {
        match Self::get_by_address(t, address).await {
            Ok(user) => Ok(user),
            Err(sqlx::Error::RowNotFound) => Self::insert_with_address(t, address).await,
            Err(err) => Err(err),
        }
    }

    pub async fn get_by_id<'t>(t: &'t mut Transaction<'_, Postgres>, id: Uuid) -> Result<User> {
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
        .fetch_one(t.as_mut())
        .await
    }
}
