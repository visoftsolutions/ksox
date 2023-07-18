use std::pin::Pin;

use fraction::Fraction;
use futures::Stream;
use sqlx::{postgres::PgPool, Result};
use uuid::Uuid;

use super::notifications::{
    NotificationManagerOutput, NotificationManagerPredicateInput, NotificationManagerSubscriber,
};
use crate::database::projections::withdraw::Withdraw;
use evm::txhash::TxHash;

#[derive(Debug, Clone)]
pub struct WithdrawsManager {
    database: PgPool,
}

impl WithdrawsManager {
    pub fn new(database: PgPool) -> Self {
        Self { database }
    }

    pub async fn get_modified(
        &self,
        last_modification_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<Vec<Withdraw>> {
        sqlx::query_as!(
            Withdraw,
            r#"
            SELECT
                id,
                created_at,
                last_modification_at,
                user_id,
                asset_id,
                tx_hash as "tx_hash: TxHash",
                amount as "amount: Fraction",
                confirmations as "confirmations: Fraction"
            FROM withdraws
            WHERE last_modification_at > $1
            ORDER BY last_modification_at ASC
            "#,
            last_modification_at
        )
        .fetch_all(&self.database)
        .await
    }

    pub fn get_all_for_user(
        &self,
        user_id: Uuid,
    ) -> Pin<Box<dyn Stream<Item = sqlx::Result<Withdraw>> + Send + '_>> {
        sqlx::query_as!(
            Withdraw,
            r#"
            SELECT
                id,
                created_at,
                last_modification_at,
                user_id,
                asset_id,
                tx_hash as "tx_hash: TxHash",
                amount as "amount: Fraction",
                confirmations as "confirmations: Fraction"
            FROM withdraws
            WHERE user_id = $1
            ORDER BY last_modification_at DESC
            "#,
            user_id
        )
        .fetch(&self.database)
    }
}

#[derive(Debug, Clone)]
pub struct WithdrawsNotificationManager {
    notification_manager_subscriber: NotificationManagerSubscriber,
}
impl WithdrawsNotificationManager {
    pub fn new(notification_manager_subscriber: NotificationManagerSubscriber) -> Self {
        Self {
            notification_manager_subscriber,
        }
    }

    pub async fn subscribe_for_user(
        &self,
        user_id: Uuid,
    ) -> sqlx::Result<Pin<Box<dyn Stream<Item = Vec<Withdraw>> + Send>>> {
        let p = predicates::function::function(move |input: &NotificationManagerPredicateInput| {
            match input {
                NotificationManagerPredicateInput::Withdraws(withdraw) => {
                    withdraw.user_id == user_id
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
                    if let NotificationManagerOutput::Withdraws(withdraw) = notification {
                        yield withdraw;
                    }
                }
            };
            Ok(Box::pin(stream))
        } else {
            Err(sqlx::Error::RowNotFound)
        }
    }
}