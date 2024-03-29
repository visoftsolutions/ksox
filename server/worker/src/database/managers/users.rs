use std::pin::Pin;

use chrono::Utc;
use evm::address::Address;
use sqlx::{
    postgres::{PgPool, PgQueryResult},
    Result,
};
use tokio_stream::Stream;
use uuid::Uuid;

use super::notifications::{
    NotificationManagerOutput, NotificationManagerPredicateInput, NotificationManagerSubscriber,
};
use crate::database::projections::user::User;

#[derive(Debug, Clone)]
pub struct UsersManager {
    database: PgPool,
}

impl UsersManager {
    pub fn new(database: PgPool) -> Self {
        UsersManager { database }
    }

    pub async fn get_for_evm_address(&self, address: Address) -> Result<User> {
        sqlx::query_as!(
            User,
            r#"
            SELECT
                users.id,
                users.created_at,
                users.last_modification_at,
                users.address as "address: Address",
                users.name,
                users.phone,
                users.email
            FROM users
            WHERE users.address = $1
            "#,
            &address.to_string()
        )
        .fetch_one(&self.database)
        .await
    }

    pub async fn insert_with_evmaddress(&self, address: Address) -> Result<User> {
        sqlx::query_as!(
            User,
            r#"
            INSERT INTO 
                users
                (last_modification_at, address) VALUES ($1, $2)
                RETURNING id, created_at, last_modification_at, address as "address: Address", users.name, users.phone, users.email
            "#,
            Utc::now(),
            &address.to_string()
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
                users.address as "address: Address",
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
                users.address as "address: Address",
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
                users.address as "address: Address",
                users.name,
                users.phone,
                users.email
            FROM users
            "#
        )
        .fetch(&self.database)
    }

    pub async fn update(&self, user: User) -> sqlx::Result<PgQueryResult> {
        let now = Utc::now();
        sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET
                last_modification_at = $2,
                name = $3,
                phone = $4,
                email = $5
            WHERE users.id = $1
            "#,
            user.id,
            now,
            user.name,
            user.phone,
            user.email,
        )
        .execute(&self.database)
        .await
    }
}

#[derive(Debug, Clone)]
pub struct UsersNotificationManager {
    notification_manager_subscriber: NotificationManagerSubscriber,
}
impl UsersNotificationManager {
    pub fn new(notification_manager_subscriber: NotificationManagerSubscriber) -> Self {
        Self {
            notification_manager_subscriber,
        }
    }

    pub async fn subscribe_to_user(
        &self,
        user_id: Uuid,
    ) -> sqlx::Result<Pin<Box<dyn Stream<Item = Vec<User>> + Send>>> {
        let p = predicates::function::function(move |input: &NotificationManagerPredicateInput| {
            match input {
                NotificationManagerPredicateInput::Users(user) => user.id == user_id,
                _ => false,
            }
        });

        if let Ok(mut rx) = self
            .notification_manager_subscriber
            .subscribe_to(Box::new(p))
            .await
        {
            let stream = async_stream::stream! {
                while let Some(notification) = rx.recv().await {
                    if let NotificationManagerOutput::Users(users) = notification {
                        yield users;
                    }
                }
            };
            Ok(Box::pin(stream))
        } else {
            Err(sqlx::Error::RowNotFound)
        }
    }
}
