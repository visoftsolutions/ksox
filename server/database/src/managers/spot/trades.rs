use std::pin::Pin;

use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use futures::Stream;
use sqlx::{
    postgres::{PgPool, PgQueryResult},
    types::Uuid,
};
use thiserror::Error;

use crate::{
    managers::notifications::{
        NotificationManagerOutput, NotificationManagerPredicateInput, NotificationManagerSubscriber,
    },
    projections::spot::trade::Trade,
    traits::{get_modified::GetModified, table_manager::TableManager},
    types::{SubscribeStream, Volume},
};

#[derive(Debug, Clone)]
pub struct TradesManager {
    database: PgPool,
}

impl TradesManager {
    pub fn new(database: PgPool) -> Self {
        TradesManager { database }
    }

    pub async fn get_after_last_modification_at(
        &self,
        last_modification_at: DateTime<Utc>,
    ) -> sqlx::Result<Option<Trade>> {
        sqlx::query_as!(
            Trade,
            r#"
            SELECT
                id,
                created_at,
                last_modification_at,
                quote_asset_id,
                base_asset_id,
                taker_id,
                order_id,
                taker_quote_volume as "taker_quote_volume: Volume",
                taker_base_volume as "taker_base_volume: Volume",
                maker_quote_volume as "maker_quote_volume: Volume",
                maker_base_volume as "maker_base_volume: Volume"
            FROM spot.trades
            WHERE (last_modification_at > $1)
            ORDER BY last_modification_at ASC
            LIMIT 1
            "#,
            last_modification_at
        )
        .fetch_optional(&self.database)
        .await
    }

    pub async fn get_last_for_asset_pair(
        &self,
        quote_asset_id: Uuid,
        base_asset_id: Uuid,
    ) -> sqlx::Result<Option<Trade>> {
        sqlx::query_as!(
            Trade,
            r#"
            SELECT
                id,
                created_at,
                last_modification_at,
                quote_asset_id,
                base_asset_id,
                taker_id,
                order_id,
                taker_quote_volume as "taker_quote_volume: Volume",
                taker_base_volume as "taker_base_volume: Volume",
                maker_quote_volume as "maker_quote_volume: Volume",
                maker_base_volume as "maker_base_volume: Volume"
            FROM spot.trades
            WHERE (quote_asset_id = $1 AND base_asset_id = $2) OR (quote_asset_id = $2 AND base_asset_id = $1)
            ORDER BY created_at DESC
            LIMIT 1
            "#,
            quote_asset_id,
            base_asset_id
        )
        .fetch_optional(&self.database)
        .await
    }

    pub fn get_after_for_asset_pair(
        &self,
        quote_asset_id: Uuid,
        base_asset_id: Uuid,
        time: DateTime<Utc>,
    ) -> Pin<Box<dyn Stream<Item = sqlx::Result<Trade>> + Send + '_>> {
        sqlx::query_as!(
            Trade,
            r#"
            SELECT
                id,
                created_at,
                last_modification_at,
                quote_asset_id,
                base_asset_id,
                taker_id,
                order_id,
                taker_quote_volume as "taker_quote_volume: Volume",
                taker_base_volume as "taker_base_volume: Volume",
                maker_quote_volume as "maker_quote_volume: Volume",
                maker_base_volume as "maker_base_volume: Volume"
            FROM spot.trades
            WHERE ((quote_asset_id = $1 AND base_asset_id = $2) OR (quote_asset_id = $2 AND base_asset_id = $1)) AND created_at >= $3
            ORDER BY created_at
            "#,
            quote_asset_id,
            base_asset_id,
            time,
        )
        .fetch(&self.database)
    }

    pub fn get_between_for_asset_pair(
        &self,
        quote_asset_id: Uuid,
        base_asset_id: Uuid,
        topen: DateTime<Utc>,
        tclose: DateTime<Utc>,
    ) -> Pin<Box<dyn Stream<Item = sqlx::Result<Trade>> + Send + '_>> {
        sqlx::query_as!(
            Trade,
            r#"
            SELECT
                id,
                created_at,
                last_modification_at,
                quote_asset_id,
                base_asset_id,
                taker_id,
                order_id,
                taker_quote_volume as "taker_quote_volume: Volume",
                taker_base_volume as "taker_base_volume: Volume",
                maker_quote_volume as "maker_quote_volume: Volume",
                maker_base_volume as "maker_base_volume: Volume"
            FROM spot.trades
            WHERE ((quote_asset_id = $1 AND base_asset_id = $2) OR (quote_asset_id = $2 AND base_asset_id = $1)) AND created_at >= $3 AND created_at < $4
            "#,
            quote_asset_id,
            base_asset_id,
            topen,
            tclose,
        )
        .fetch(&self.database)
    }

    pub fn get_ordered_asc(
        &self,
        user_id: Uuid,
    ) -> Pin<Box<dyn Stream<Item = sqlx::Result<Trade>> + Send + '_>> {
        sqlx::query_as!(
            Trade,
            r#"
            SELECT
                spot.trades.id,
                spot.trades.created_at,
                spot.trades.last_modification_at,
                spot.trades.quote_asset_id,
                spot.trades.base_asset_id,
                spot.trades.taker_id,
                spot.trades.order_id,
                spot.trades.taker_quote_volume as "taker_quote_volume: Volume",
                spot.trades.taker_base_volume as "taker_base_volume: Volume",
                spot.trades.maker_quote_volume as "maker_quote_volume: Volume",
                spot.trades.maker_base_volume as "maker_base_volume: Volume"
            FROM spot.trades
            JOIN spot.orders ON spot.trades.order_id = spot.orders.id
            WHERE spot.orders.user_id = $1
            ORDER BY created_at ASC
            "#,
            user_id
        )
        .fetch(&self.database)
    }

    pub fn get_ordered_desc(
        &self,
        user_id: Uuid,
    ) -> Pin<Box<dyn Stream<Item = sqlx::Result<Trade>> + Send + '_>> {
        sqlx::query_as!(
            Trade,
            r#"
            SELECT
                spot.trades.id,
                spot.trades.created_at,
                spot.trades.last_modification_at,
                spot.trades.quote_asset_id,
                spot.trades.base_asset_id,
                spot.trades.taker_id,
                spot.trades.order_id,
                spot.trades.taker_quote_volume as "taker_quote_volume: Volume",
                spot.trades.taker_base_volume as "taker_base_volume: Volume",
                spot.trades.maker_quote_volume as "maker_quote_volume: Volume",
                spot.trades.maker_base_volume as "maker_base_volume: Volume"
            FROM spot.trades
            JOIN spot.orders ON spot.trades.order_id = spot.orders.id
            WHERE spot.orders.user_id = $1
            ORDER BY created_at DESC
            "#,
            user_id
        )
        .fetch(&self.database)
    }

    pub fn get_for_taker(
        &self,
        taker_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Pin<Box<dyn Stream<Item = sqlx::Result<Trade>> + Send + '_>> {
        sqlx::query_as!(
            Trade,
            r#"
            SELECT
                id,
                created_at,
                last_modification_at,
                quote_asset_id,
                base_asset_id,
                taker_id,
                order_id,
                spot.trades.taker_quote_volume as "taker_quote_volume: Volume",
                spot.trades.taker_base_volume as "taker_base_volume: Volume",
                spot.trades.maker_quote_volume as "maker_quote_volume: Volume",
                spot.trades.maker_base_volume as "maker_base_volume: Volume"
            FROM spot.trades
            WHERE taker_id = $1
            ORDER BY created_at DESC
            LIMIT $2
            OFFSET $3
            "#,
            taker_id,
            limit,
            offset
        )
        .fetch(&self.database)
    }

    pub fn get_for_asset_pair(
        &self,
        quote_asset_id: Uuid,
        base_asset_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Pin<Box<dyn Stream<Item = sqlx::Result<Trade>> + Send + '_>> {
        sqlx::query_as!(
            Trade,
            r#"
            SELECT
                id,
                created_at,
                last_modification_at,
                quote_asset_id,
                base_asset_id,
                taker_id,
                order_id,
                taker_quote_volume as "taker_quote_volume: Volume",
                taker_base_volume as "taker_base_volume: Volume",
                maker_quote_volume as "maker_quote_volume: Volume",
                maker_base_volume as "maker_base_volume: Volume"
            FROM spot.trades
            WHERE (quote_asset_id = $1 AND base_asset_id = $2) OR (quote_asset_id = $2 AND base_asset_id = $1)
            ORDER BY created_at DESC
            LIMIT $3
            OFFSET $4
            "#,
            quote_asset_id,
            base_asset_id,
            limit,
            offset
        )
        .fetch(&self.database)
    }
}

impl TableManager<Trade> for TradesManager {
    fn get_all(&self) -> Pin<Box<dyn Stream<Item = sqlx::Result<Trade>> + Send + '_>> {
        sqlx::query_as!(
            Trade,
            r#"
            SELECT
                id,
                created_at,
                last_modification_at,
                quote_asset_id,
                base_asset_id,
                taker_id,
                order_id,
                spot.trades.taker_quote_volume as "taker_quote_volume: Volume",
                spot.trades.taker_base_volume as "taker_base_volume: Volume",
                spot.trades.maker_quote_volume as "maker_quote_volume: Volume",
                spot.trades.maker_base_volume as "maker_base_volume: Volume"
            FROM spot.trades
            "#
        )
        .fetch(&self.database)
    }

    async fn get_by_id(&self, id: Uuid) -> sqlx::Result<Trade> {
        sqlx::query_as!(
            Trade,
            r#"
            SELECT
                id,
                created_at,
                last_modification_at,
                quote_asset_id,
                base_asset_id,
                taker_id,
                order_id,
                spot.trades.taker_quote_volume as "taker_quote_volume: Volume",
                spot.trades.taker_base_volume as "taker_base_volume: Volume",
                spot.trades.maker_quote_volume as "maker_quote_volume: Volume",
                spot.trades.maker_base_volume as "maker_base_volume: Volume"
            FROM spot.trades
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.database)
        .await
    }

    async fn insert(&self, element: Trade) -> sqlx::Result<PgQueryResult> {
        let taker_quote_volume: BigDecimal = element.taker_quote_volume.into();
        let maker_quote_volume: BigDecimal = element.maker_quote_volume.into();
        let taker_base_volume: BigDecimal = element.taker_base_volume.into();
        let maker_base_volume: BigDecimal = element.maker_base_volume.into();
        sqlx::query!(
            r#"
            INSERT INTO
                spot.trades
                (id, created_at, quote_asset_id, base_asset_id, taker_id, order_id, taker_quote_volume, maker_quote_volume, taker_base_volume, maker_base_volume)
            VALUES
                ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#,
            element.id,
            element.created_at,
            element.quote_asset_id,
            element.base_asset_id,
            element.taker_id,
            element.order_id,
            taker_quote_volume,
            maker_quote_volume,
            taker_base_volume,
            maker_base_volume
        )
        .execute(&self.database)
        .await
    }

    async fn update(&self, element: Trade) -> sqlx::Result<PgQueryResult> {
        let taker_quote_volume: BigDecimal = element.taker_quote_volume.into();
        let maker_quote_volume: BigDecimal = element.maker_quote_volume.into();
        let taker_base_volume: BigDecimal = element.taker_base_volume.into();
        let maker_base_volume: BigDecimal = element.maker_base_volume.into();
        sqlx::query!(
            r#"
            UPDATE 
                spot.trades 
            SET
                created_at = $2,
                quote_asset_id = $3,
                base_asset_id = $4,
                taker_id = $5,
                order_id = $6,
                taker_quote_volume = $7,
                maker_quote_volume = $8,
                taker_base_volume = $9,
                maker_base_volume = $10
            WHERE
                id = $1
            "#,
            element.id,
            element.created_at,
            element.quote_asset_id,
            element.base_asset_id,
            element.taker_id,
            element.order_id,
            taker_quote_volume,
            maker_quote_volume,
            taker_base_volume,
            maker_base_volume
        )
        .execute(&self.database)
        .await
    }

    async fn delete(&self, element: Trade) -> sqlx::Result<PgQueryResult> {
        let mut transaction = self.database.begin().await?;
        sqlx::query!(
            r#"
            LOCK TABLE spot.trades IN ACCESS EXCLUSIVE MODE
            "#
        )
        .execute(&mut transaction)
        .await?;
        let result = sqlx::query!(
            r#"
            DELETE FROM 
                spot.trades 
            WHERE
                id = $1
            "#,
            element.id,
        )
        .execute(&mut transaction)
        .await?;
        transaction.commit().await?;
        Ok(result)
    }
}

impl GetModified<Trade> for TradesManager {
    async fn get_modified(&self, last_modification_at: DateTime<Utc>) -> sqlx::Result<Vec<Trade>> {
        sqlx::query_as!(
            Trade,
            r#"
            SELECT
                id,
                created_at,
                last_modification_at,
                quote_asset_id,
                base_asset_id,
                taker_id,
                order_id,
                spot.trades.taker_quote_volume as "taker_quote_volume: Volume",
                spot.trades.taker_base_volume as "taker_base_volume: Volume",
                spot.trades.maker_quote_volume as "maker_quote_volume: Volume",
                spot.trades.maker_base_volume as "maker_base_volume: Volume"
            FROM spot.trades
            WHERE last_modification_at > $1
            "#,
            last_modification_at
        )
        .fetch_all(&self.database)
        .await
    }
}

#[derive(Debug, Error)]
pub enum TradesNotificationManagerError {}

#[derive(Debug, Clone)]
pub struct TradesNotificationManager {
    notification_manager_subscriber: NotificationManagerSubscriber,
}
impl TradesNotificationManager {
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

    pub async fn subscribe_to_taker(
        &self,
        taker_id: Uuid,
    ) -> sqlx::Result<SubscribeStream<Vec<Trade>>> {
        let p = predicates::function::function(move |input: &NotificationManagerPredicateInput| {
            match input {
                NotificationManagerPredicateInput::SpotTradesChanged(trade) => {
                    trade.taker_id == taker_id
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
