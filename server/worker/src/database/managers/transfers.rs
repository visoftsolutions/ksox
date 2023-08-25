use std::pin::Pin;

use evm::address::Address;
use fraction::Fraction;
use futures::Stream;
use sqlx::{postgres::PgPool, Result};
use tokio_stream::StreamExt;
use uuid::Uuid;

use super::notifications::{
    NotificationManagerOutput, NotificationManagerPredicateInput, NotificationManagerSubscriber,
};
use crate::database::projections::transfer::{DisplayTransfer, Transfer};

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

    pub fn get_display_transfers_by_ids(
        &self,
        ids: &[Uuid],
    ) -> Pin<Box<dyn Stream<Item = sqlx::Result<DisplayTransfer>> + Send + '_>> {
        sqlx::query_as!(
            DisplayTransfer,
            r#"
            SELECT
                transfers.id,
                transfers.created_at,
                from_valuts.user_id as from_user_id,
                from_users.address as "from_user_address: Address",
                to_valuts.user_id as to_user_id,
                to_users.address as "to_user_address: Address",
                assets.id as asset_id,
                assets.icon_path as asset_icon_path,
                assets.name as asset_name,
                assets.symbol as asset_symbol,
                transfers.amount as "amount: Fraction",
                transfers.fee as "fee: Fraction"
            FROM transfers
            JOIN valuts from_valuts ON transfers.from_valut_id = from_valuts.id
            JOIN valuts to_valuts ON transfers.to_valut_id = to_valuts.id
            JOIN assets ON transfers.asset_id = assets.id
            JOIN users from_users ON from_valuts.user_id = from_users.id
            JOIN users to_users ON to_valuts.user_id = to_users.id
            WHERE transfers.id = ANY($1)
            ORDER BY transfers.created_at DESC
            "#,
            &ids,
        )
        .fetch(&self.database)
    }

    pub fn get_only_external_for_user_id(
        &self,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Pin<Box<dyn Stream<Item = sqlx::Result<DisplayTransfer>> + Send + '_>> {
        sqlx::query_as!(
            DisplayTransfer,
            r#"
            SELECT
                transfers.id,
                transfers.created_at,
                from_valuts.user_id as from_user_id,
                from_users.address as "from_user_address: Address",
                to_valuts.user_id as to_user_id,
                to_users.address as "to_user_address: Address",
                assets.id as asset_id,
                assets.icon_path as asset_icon_path,
                assets.name as asset_name,
                assets.symbol as asset_symbol,
                transfers.amount as "amount: Fraction",
                transfers.fee as "fee: Fraction"
            FROM transfers
            JOIN valuts from_valuts ON transfers.from_valut_id = from_valuts.id
            JOIN valuts to_valuts ON transfers.to_valut_id = to_valuts.id
            JOIN assets ON transfers.asset_id = assets.id
            JOIN users from_users ON from_valuts.user_id = from_users.id
            JOIN users to_users ON to_valuts.user_id = to_users.id
            WHERE (to_valuts.user_id = $1 AND from_valuts.user_id = '00000000-0000-0000-0000-000000000000')
                OR (to_valuts.user_id = '00000000-0000-0000-0000-000000000000' AND from_valuts.user_id = $1)
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
    transfers_manager: TransfersManager,
}
impl TransfersNotificationManager {
    pub fn new(
        notification_manager_subscriber: NotificationManagerSubscriber,
        transfers_manager: TransfersManager,
    ) -> Self {
        Self {
            notification_manager_subscriber,
            transfers_manager,
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

    pub async fn subscribe_only_external_to_user(
        &self,
        user_id: Uuid,
    ) -> sqlx::Result<Pin<Box<dyn Stream<Item = Result<DisplayTransfer>> + Send>>> {
        let p = predicates::function::function(move |input: &NotificationManagerPredicateInput| {
            match input {
                NotificationManagerPredicateInput::Transfers(transfer) => {
                    (transfer.from_user_id == user_id && transfer.to_user_id == Uuid::from_u128(0))
                        || (transfer.from_user_id == Uuid::from_u128(0)
                            && transfer.to_user_id == user_id)
                }
                _ => false,
            }
        });

        if let Ok(mut rx) = self
            .notification_manager_subscriber
            .subscribe_to(Box::new(p))
            .await
        {
            let transfers_manager = self.transfers_manager.clone();
            let stream = async_stream::try_stream! {
                while let Some(notification) = rx.recv().await {
                    if let NotificationManagerOutput::Transfers(transfers) = notification {
                        let transfers_ids: Vec<Uuid> = transfers.iter().map(|x| x.id).collect();
                        let mut results = transfers_manager.get_display_transfers_by_ids(transfers_ids.as_slice());
                        while let Some(result) = results.next().await {
                            yield result?;
                        }
                    }
                }
            };
            Ok(Box::pin(stream))
        } else {
            Err(sqlx::Error::RowNotFound)
        }
    }
}
