use std::pin::Pin;

use fraction::Fraction;
use futures::Stream;
use sqlx::{postgres::PgPool, Result};
use uuid::Uuid;

use super::notifications::{
    NotificationManagerOutput, NotificationManagerPredicateInput, NotificationManagerSubscriber,
};
use crate::database::projections::transfer::Transfer;

#[derive(Debug, Clone)]
pub struct TransfersManager {
    database: PgPool,
}

impl TransfersManager {
    pub fn new(database: PgPool) -> Self {
        Self { database }
    }

    pub async fn get_modified(
        &self,
        last_modification_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<Vec<Transfer>> {
        sqlx::query_as!(
            Transfer,
            r#"
            SELECT
                transfers.id,
                transfers.created_at,
                transfers.last_modification_at,
                transfers.from_valut_id,
                from_valuts.user_id as from_user_id,
                transfers.to_valut_id,
                to_valuts.user_id as to_user_id,
                transfers.asset_id,
                transfers.amount as "amount: Fraction",
                transfers.fee as "fee: Fraction"
            FROM transfers
            JOIN valuts from_valuts ON transfers.from_valut_id = from_valuts.id
            JOIN valuts to_valuts ON transfers.to_valut_id = to_valuts.id
            WHERE transfers.last_modification_at > $1
            ORDER BY last_modification_at ASC
            "#,
            last_modification_at
        )
        .fetch_all(&self.database)
        .await
    }

    pub fn get_all(&self) -> Pin<Box<dyn Stream<Item = sqlx::Result<Transfer>> + Send + '_>> {
        sqlx::query_as!(
            Transfer,
            r#"
            SELECT
                transfers.id,
                transfers.created_at,
                transfers.last_modification_at,
                transfers.from_valut_id,
                from_valuts.user_id as from_user_id,
                transfers.to_valut_id,
                to_valuts.user_id as to_user_id,
                transfers.asset_id,
                transfers.amount as "amount: Fraction",
                transfers.fee as "fee: Fraction"
            FROM transfers
            JOIN valuts from_valuts ON transfers.from_valut_id = from_valuts.id
            JOIN valuts to_valuts ON transfers.to_valut_id = to_valuts.id
            "#
        )
        .fetch(&self.database)
    }

    pub fn get_for_user_id(
        &self,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Pin<Box<dyn Stream<Item = sqlx::Result<Transfer>> + Send + '_>> {
        sqlx::query_as!(
            Transfer,
            r#"
            SELECT
                transfers.id,
                transfers.created_at,
                transfers.last_modification_at,
                transfers.from_valut_id,
                from_valuts.user_id as from_user_id,
                transfers.to_valut_id,
                to_valuts.user_id as to_user_id,
                transfers.asset_id,
                transfers.amount as "amount: Fraction",
                transfers.fee as "fee: Fraction"
            FROM transfers
            JOIN valuts from_valuts ON transfers.from_valut_id = from_valuts.id
            JOIN valuts to_valuts ON transfers.to_valut_id = to_valuts.id
            WHERE transfers.to_valut_id = $1 OR transfers.from_valut_id = $1
            ORDER BY transfers.created_at DESC
            LIMIT $2
            OFFSET $3
            "#,
            user_id,
            limit,
            offset
        )
        .fetch(&self.database)
    }
}

#[derive(Debug, Clone)]
pub struct TransfersNotificationManager {
    notification_manager_subscriber: NotificationManagerSubscriber,
}
impl TransfersNotificationManager {
    pub fn new(notification_manager_subscriber: NotificationManagerSubscriber) -> Self {
        Self {
            notification_manager_subscriber,
        }
    }

    pub async fn subscribe_to_user_as_taker(
        &self,
        user_id: Uuid,
    ) -> sqlx::Result<Pin<Box<dyn Stream<Item = Vec<Transfer>> + Send>>> {
        let p = predicates::function::function(move |input: &NotificationManagerPredicateInput| {
            match input {
                NotificationManagerPredicateInput::Transfers(transfer) => {
                    transfer.to_user_id == user_id
                }
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
                    if let NotificationManagerOutput::Transfers(transfers) = notification {
                        yield transfers;
                    }
                }
            };
            Ok(Box::pin(stream))
        } else {
            Err(sqlx::Error::RowNotFound)
        }
    }

    pub async fn subscribe_to_user(
        &self,
        user_id: Uuid,
    ) -> sqlx::Result<Pin<Box<dyn Stream<Item = Vec<Transfer>> + Send>>> {
        let p = predicates::function::function(move |input: &NotificationManagerPredicateInput| {
            match input {
                NotificationManagerPredicateInput::Transfers(transfer) => {
                    transfer.from_user_id == user_id || transfer.to_user_id == user_id
                }
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
                    if let NotificationManagerOutput::Transfers(transfers) = notification {
                        yield transfers;
                    }
                }
            };
            Ok(Box::pin(stream))
        } else {
            Err(sqlx::Error::RowNotFound)
        }
    }
}
