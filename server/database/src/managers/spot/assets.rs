use std::pin::Pin;

use futures::Stream;
use sqlx::{
    postgres::{PgPool, PgQueryResult},
    types::Uuid,
    Result,
};

use crate::{
    managers::notifications::{
        NotificationManagerOutput, NotificationManagerPredicateInput, NotificationManagerSubscriber,
    },
    projections::spot::{asset::Asset, trade::Trade},
    traits::{get_modified::GetModified, table_manager::TableManager},
    types::{Fraction, SubscribeStream},
};

#[derive(Debug, Clone)]
pub struct AssetsManager {
    database: PgPool,
}

impl AssetsManager {
    pub fn new(database: PgPool) -> Self {
        Self { database }
    }
}

impl TableManager<Asset> for AssetsManager {
    fn get_all(&self) -> Pin<Box<dyn Stream<Item = Result<Asset>> + Send + '_>> {
        sqlx::query_as!(
            Asset,
            r#"
            SELECT
                id,
                created_at,
                last_modification_at,
                name,
                symbol,
                maker_fee as "maker_fee: Fraction",
                taker_fee as "taker_fee: Fraction"
            FROM spot.assets
            "#
        )
        .fetch(&self.database)
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Asset> {
        sqlx::query_as!(
            Asset,
            r#"
            SELECT
                id,
                created_at,
                last_modification_at,
                name,
                symbol,
                maker_fee as "maker_fee: Fraction",
                taker_fee as "taker_fee: Fraction"
            FROM spot.assets
            WHERE spot.assets.id = $1
            "#,
            id
        )
        .fetch_one(&self.database)
        .await
    }

    async fn insert(&self, element: Asset) -> Result<PgQueryResult> {
        sqlx::query!(
            r#"
            INSERT INTO 
                spot.assets 
                (id, created_at, last_modification_at, name, symbol, maker_fee, taker_fee)
            VALUES
                ($1, $2, $3, $4, $5, $6::fraction, $7::fraction)
            "#,
            element.id,
            element.created_at,
            chrono::Utc::now(),
            element.name,
            element.symbol,
            element.maker_fee.to_string() as _,
            element.taker_fee.to_string() as _
        )
        .execute(&self.database)
        .await
    }

    async fn update(&self, element: Asset) -> Result<PgQueryResult> {
        sqlx::query!(
            r#"
            UPDATE 
                spot.assets 
            SET
                last_modification_at = $2,
                name = $3,
                symbol = $4,
                maker_fee = $5,
                taker_fee = $6
            WHERE
                id = $1
            "#,
            element.id,
            chrono::Utc::now(),
            element.name,
            element.symbol,
            element.maker_fee.to_string() as _,
            element.taker_fee.to_string() as _
        )
        .execute(&self.database)
        .await
    }

    async fn delete(&self, element: Asset) -> Result<PgQueryResult> {
        sqlx::query!(
            r#"
            DELETE FROM 
                spot.assets 
            WHERE
                id = $1
            "#,
            element.id,
        )
        .execute(&self.database)
        .await
    }
}

impl GetModified<Asset> for AssetsManager {
    async fn get_modified(
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
                symbol,
                maker_fee as "maker_fee: Fraction",
                taker_fee as "taker_fee: Fraction"
            FROM spot.assets
            WHERE last_modification_at > $1
            "#,
            last_modification_at
        )
        .fetch_all(&self.database)
        .await
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
    ) -> sqlx::Result<SubscribeStream<Vec<Trade>>> {
        let p = predicates::function::function(move |input: &NotificationManagerPredicateInput| {
            match input {
                NotificationManagerPredicateInput::SpotTradesChanged(trade) => {
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
                    if let NotificationManagerOutput::SpotTradesChanged(trades) = notification {
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
