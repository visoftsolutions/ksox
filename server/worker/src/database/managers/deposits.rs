use std::pin::Pin;

use evm::address::Address;
use fraction::Fraction;
use futures::Stream;
use sqlx::{postgres::PgPool, Result};

use super::notifications::{
    NotificationManagerOutput, NotificationManagerPredicateInput, NotificationManagerSubscriber,
};
use crate::database::projections::deposit::Deposit;
use evm::txhash::TxHash;

#[derive(Debug, Clone)]
pub struct DepositsManager {
    database: PgPool,
}

impl DepositsManager {
    pub fn new(database: PgPool) -> Self {
        Self { database }
    }

    pub async fn get_modified(
        &self,
        last_modification_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<Vec<Deposit>> {
        sqlx::query_as!(
            Deposit,
            r#"
            SELECT
                id,
                created_at,
                last_modification_at,
                owner as "owner: Address",
                spender as "spender: Address",
                asset as "asset: Address",
                amount as "amount: Fraction",
                tx_hash as "tx_hash: TxHash",
                confirmations as "confirmations: Fraction"
            FROM deposits
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
        user_address: Address,
    ) -> Pin<Box<dyn Stream<Item = sqlx::Result<Deposit>> + Send + '_>> {
        sqlx::query_as!(
            Deposit,
            r#"
            SELECT
                id,
                created_at,
                last_modification_at,
                owner as "owner: Address",
                spender as "spender: Address",
                asset as "asset: Address",
                amount as "amount: Fraction",
                tx_hash as "tx_hash: TxHash",
                confirmations as "confirmations: Fraction"
            FROM deposits
            WHERE spender = $1
            ORDER BY last_modification_at DESC
            "#,
            user_address.to_string() as _
        )
        .fetch(&self.database)
    }
}

#[derive(Debug, Clone)]
pub struct DepositsNotificationManager {
    notification_manager_subscriber: NotificationManagerSubscriber,
}
impl DepositsNotificationManager {
    pub fn new(notification_manager_subscriber: NotificationManagerSubscriber) -> Self {
        Self {
            notification_manager_subscriber,
        }
    }

    pub async fn subscribe_for_user(
        &self,
        user_address: Address,
    ) -> sqlx::Result<Pin<Box<dyn Stream<Item = Vec<Deposit>> + Send>>> {
        let p = predicates::function::function(move |input: &NotificationManagerPredicateInput| {
            match input {
                NotificationManagerPredicateInput::Deposits(deposit) => {
                    deposit.spender == user_address
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
                    if let NotificationManagerOutput::Deposits(deposits) = notification {
                        yield deposits;
                    }
                }
            };
            Ok(Box::pin(stream))
        } else {
            Err(sqlx::Error::RowNotFound)
        }
    }
}
