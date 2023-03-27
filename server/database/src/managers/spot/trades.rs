use std::pin::Pin;

use bigdecimal::BigDecimal;
use bytes::Bytes;
use chrono::{DateTime, Utc};
use futures::{Stream, StreamExt};
use sqlx::{
    postgres::{PgListener, PgPool, PgQueryResult},
    types::Uuid,
    Result,
};
use tokio::{sync::oneshot, task};

use crate::{
    projections::spot::trade::Trade,
    traits::table_manager::TableManager,
    types::{NotifyTrigger, SubscribeStream, Volume},
    utils::trigger_name,
};

#[derive(Debug, Clone)]
pub struct TradesManager {
    database: PgPool,
}

impl TradesManager {
    pub fn new(database: PgPool) -> Self {
        TradesManager { database }
    }

    pub async fn get_last_for_asset_pair(
        &self,
        quote_asset_id: Uuid,
        base_asset_id: Uuid,
    ) -> Result<Option<Trade>> {
        sqlx::query_as!(
            Trade,
            r#"
            SELECT
                id,
                created_at,
                quote_asset_id,
                base_asset_id,
                taker_id,
                order_id,
                taker_quote_volume as "taker_quote_volume: Volume",
                taker_base_volume as "taker_base_volume: Volume",
                maker_quote_volume as "maker_quote_volume: Volume",
                maker_base_volume as "maker_base_volume: Volume"
            FROM spot.trades
            WHERE quote_asset_id = $1 AND base_asset_id = $2
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
    ) -> Pin<Box<dyn Stream<Item = Result<Trade>> + Send + '_>> {
        sqlx::query_as!(
            Trade,
            r#"
            SELECT
                id,
                created_at,
                quote_asset_id,
                base_asset_id,
                taker_id,
                order_id,
                taker_quote_volume as "taker_quote_volume: Volume",
                taker_base_volume as "taker_base_volume: Volume",
                maker_quote_volume as "maker_quote_volume: Volume",
                maker_base_volume as "maker_base_volume: Volume"
            FROM spot.trades
            WHERE quote_asset_id = $1 AND base_asset_id = $2 AND created_at >= $3
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
    ) -> Pin<Box<dyn Stream<Item = Result<Trade>> + Send + '_>> {
        sqlx::query_as!(
            Trade,
            r#"
            SELECT
                id,
                created_at,
                quote_asset_id,
                base_asset_id,
                taker_id,
                order_id,
                taker_quote_volume as "taker_quote_volume: Volume",
                taker_base_volume as "taker_base_volume: Volume",
                maker_quote_volume as "maker_quote_volume: Volume",
                maker_base_volume as "maker_base_volume: Volume"
            FROM spot.trades
            WHERE quote_asset_id = $1 AND base_asset_id = $2
            AND created_at >= $3 AND created_at < $4
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
    ) -> Pin<Box<dyn Stream<Item = Result<Trade>> + Send + '_>> {
        sqlx::query_as!(
            Trade,
            r#"
            SELECT
                spot.trades.id,
                spot.trades.created_at,
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
    ) -> Pin<Box<dyn Stream<Item = Result<Trade>> + Send + '_>> {
        sqlx::query_as!(
            Trade,
            r#"
            SELECT
                spot.trades.id,
                spot.trades.created_at,
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
    ) -> Pin<Box<dyn Stream<Item = Result<Trade>> + Send + '_>> {
        sqlx::query_as!(
            Trade,
            r#"
            SELECT
                id,
                created_at,
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
    ) -> Pin<Box<dyn Stream<Item = Result<Trade>> + Send + '_>> {
        sqlx::query_as!(
            Trade,
            r#"
            SELECT
                id,
                created_at,
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

    pub async fn create_notify_trigger_for_taker(&self, taker_id: Uuid) -> Result<NotifyTrigger> {
        let trigger_name = trigger_name(
            "spot_trades_notify_trigger_for_taker",
            vec![Bytes::from(taker_id.as_bytes().to_owned().to_vec())],
        );
        sqlx::query!(
            r#"
            SELECT create_spot_trades_notify_trigger_for_taker($1, $2)
            "#,
            trigger_name,
            taker_id
        )
        .execute(&self.database)
        .await?;

        let db = self.database.clone();
        let trigger_name_clone = trigger_name.clone();
        let (tx, rx) = oneshot::channel::<()>();
        task::spawn(async move {
            if (rx.await).is_err() {
                tracing::error!("drop_signal failed");
            }
            if let Err(err) = sqlx::query!(
                r#"
                SELECT drop_spot_trades_notify_trigger_for_taker($1)
                "#,
                trigger_name_clone
            )
            .execute(&db)
            .await
            {
                tracing::error!("{err}");
            }
        });

        Ok(NotifyTrigger::new(format!("c_{trigger_name}"), tx))
    }

    pub async fn subscribe_for_taker(&self, taker_id: Uuid) -> Result<SubscribeStream<Trade>> {
        let mut listener = PgListener::connect_with(&self.database).await?;
        let notify_trigger = self.create_notify_trigger_for_taker(taker_id).await?;
        listener.listen(&notify_trigger.channel_name).await?;

        let subscribe_stream = listener.into_stream().map(|element| {
            element.and_then(|val| {
                serde_json::from_str::<Trade>(val.payload())
                    .map_err(|err| sqlx::Error::from(std::io::Error::from(err)))
            })
        });

        Ok(SubscribeStream::new(
            notify_trigger,
            Box::pin(subscribe_stream),
        ))
    }

    pub async fn create_notify_trigger_for_asset_pair(
        &self,
        quote_asset_id: Uuid,
        base_asset_id: Uuid,
    ) -> Result<NotifyTrigger> {
        let trigger_name = trigger_name(
            "spot_trades_notify_trigger_for_asset_pair",
            vec![
                Bytes::from(quote_asset_id.as_bytes().to_owned().to_vec()),
                Bytes::from(base_asset_id.as_bytes().to_owned().to_vec()),
            ],
        );
        sqlx::query!(
            r#"
            SELECT create_spot_trades_notify_trigger_for_asset_pair($1, $2, $3)
            "#,
            trigger_name,
            quote_asset_id,
            base_asset_id
        )
        .execute(&self.database)
        .await?;

        let db = self.database.clone();
        let trigger_name_clone = trigger_name.clone();
        let (tx, rx) = oneshot::channel::<()>();
        task::spawn(async move {
            if (rx.await).is_err() {
                tracing::error!("drop_signal failed");
            }
            if let Err(err) = sqlx::query!(
                r#"
                SELECT drop_spot_trades_notify_trigger_for_asset_pair($1)
                "#,
                trigger_name_clone
            )
            .execute(&db)
            .await
            {
                tracing::error!("{err}");
            }
        });

        Ok(NotifyTrigger::new(format!("c_{trigger_name}"), tx))
    }

    pub async fn subscribe_for_asset_pair(
        &self,
        quote_asset_id: Uuid,
        base_asset_id: Uuid,
    ) -> Result<SubscribeStream<Trade>> {
        let mut listener = PgListener::connect_with(&self.database).await?;
        let notify_trigger = self
            .create_notify_trigger_for_asset_pair(quote_asset_id, base_asset_id)
            .await?;
        listener.listen(&notify_trigger.channel_name).await?;

        let subscribe_stream = listener.into_stream().map(|element| {
            element.and_then(|val| {
                serde_json::from_str::<Trade>(val.payload())
                    .map_err(|err| sqlx::Error::from(std::io::Error::from(err)))
            })
        });

        Ok(SubscribeStream::new(
            notify_trigger,
            Box::pin(subscribe_stream),
        ))
    }
}

impl TableManager<Trade> for TradesManager {
    fn get_all(&self) -> Pin<Box<dyn Stream<Item = Result<Trade>> + Send + '_>> {
        sqlx::query_as!(
            Trade,
            r#"
            SELECT
                id,
                created_at,
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

    async fn get_by_id(&self, id: Uuid) -> Result<Trade> {
        sqlx::query_as!(
            Trade,
            r#"
            SELECT
                id,
                created_at,
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

    async fn insert(&self, element: Trade) -> Result<PgQueryResult> {
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

    async fn update(&self, element: Trade) -> Result<PgQueryResult> {
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

    async fn delete(&self, element: Trade) -> Result<PgQueryResult> {
        sqlx::query!(
            r#"
            DELETE FROM 
                spot.trades 
            WHERE
                id = $1
            "#,
            element.id,
        )
        .execute(&self.database)
        .await
    }
}
