use std::pin::Pin;

use evm::address::Address;
use fraction::Fraction;
use futures::Stream;
use sqlx::{postgres::PgPool, Result};
use uuid::Uuid;

use super::notifications::{
    NotificationManagerOutput, NotificationManagerPredicateInput, NotificationManagerSubscriber,
};
use crate::database::projections::{asset::Asset, trade::Trade};

#[derive(Debug, Clone)]
pub struct AssetsManager {
    database: PgPool,
}

impl AssetsManager {
    pub fn new(database: PgPool) -> Self {
        Self { database }
    }

    pub async fn get_by_id(&self, asset_id: Uuid) -> Result<Asset> {
        sqlx::query_as!(
            Asset,
            r#"
            SELECT
                id,
                created_at,
                last_modification_at,
                name,
                icon_path,
                symbol,
                address as "address: Address",
                decimals as "decimals: Fraction",
                maker_fee as "maker_fee: Fraction",
                taker_fee as "taker_fee: Fraction",
                transfer_fee as "transfer_fee: Fraction"
            FROM assets
            WHERE id = $1
            "#,
            asset_id
        )
        .fetch_one(&self.database)
        .await
    }

    pub async fn get_modified(
        &self,
        last_modification_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<Vec<Asset>> {
        sqlx::query_as!(
            Asset,
            r#"
            SELECT
                id,
                created_at,
                last_modification_at,
                name,
                icon_path,
                symbol,
                address as "address: Address",
                decimals as "decimals: Fraction",
                maker_fee as "maker_fee: Fraction",
                taker_fee as "taker_fee: Fraction",
                transfer_fee as "transfer_fee: Fraction"
            FROM assets
            WHERE last_modification_at > $1
            ORDER BY last_modification_at ASC
            "#,
            last_modification_at
        )
        .fetch_all(&self.database)
        .await
    }

    pub fn get_all(&self) -> Pin<Box<dyn Stream<Item = sqlx::Result<Asset>> + Send + '_>> {
        sqlx::query_as!(
            Asset,
            r#"
            SELECT
                id,
                created_at,
                last_modification_at,
                name,
                icon_path,
                symbol,
                address as "address: Address",
                decimals as "decimals: Fraction",
                maker_fee as "maker_fee: Fraction",
                taker_fee as "taker_fee: Fraction",
                transfer_fee as "transfer_fee: Fraction"
            FROM assets
            "#
        )
        .fetch(&self.database)
    }
}

#[derive(Debug, Clone)]
pub struct AssetsNotificationManager {
    notification_manager_subscriber: NotificationManagerSubscriber,
}
impl AssetsNotificationManager {
    pub fn new(notification_manager_subscriber: NotificationManagerSubscriber) -> Self {
        Self {
            notification_manager_subscriber,
        }
    }

    pub async fn subscribe_to_asset_pair(
        &self,
        quote_asset_id: Uuid,
        base_asset_id: Uuid,
    ) -> sqlx::Result<Pin<Box<dyn Stream<Item = Vec<Trade>> + Send>>> {
        let p = predicates::function::function(move |input: &NotificationManagerPredicateInput| {
            match input {
                NotificationManagerPredicateInput::SpotTrades(trade) => {
                    (trade.quote_asset_id == quote_asset_id && trade.base_asset_id == base_asset_id)
                        || (trade.quote_asset_id == base_asset_id
                            && trade.base_asset_id == quote_asset_id)
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
                    if let NotificationManagerOutput::SpotTrades(trades) = notification {
                        yield trades;
                    }
                }
            };
            Ok(Box::pin(stream))
        } else {
            Err(sqlx::Error::RowNotFound)
        }
    }
}
