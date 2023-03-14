use std::pin::Pin;

use bigdecimal::BigDecimal;
use bytes::Bytes;
use futures::{Stream, StreamExt};
use sqlx::{
    postgres::{PgListener, PgPool, PgQueryResult},
    types::Uuid,
    Result,
};
use tokio::{sync::oneshot, task};

use crate::{
    projections::spot::valut::Valut,
    traits::table_manager::TableManager,
    types::{NotifyTrigger, SubscribeStream, Volume},
    utils::trigger_name,
};

#[derive(Debug, Clone)]
pub struct ValutsManager {
    database: PgPool,
}

impl ValutsManager {
    pub fn new(database: PgPool) -> Self {
        ValutsManager { database }
    }

    pub fn get_for_user(
        &self,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Pin<Box<dyn Stream<Item = Result<Valut>> + Send + '_>> {
        sqlx::query_as!(
            Valut,
            r#"
            SELECT
                id,
                user_id,
                asset_id,
                balance as "balance: Volume"
            FROM spot.valuts
            WHERE user_id = $1
            ORDER BY balance
            LIMIT $2
            OFFSET $3
            "#,
            user_id,
            limit,
            offset
        )
        .fetch(&self.database)
    }

    pub async fn get_for_user_and_asset(&self, user_id: Uuid, asset_id: Uuid) -> Result<Valut> {
        sqlx::query_as!(
            Valut,
            r#"
            SELECT
                id,
                user_id,
                asset_id,
                balance as "balance: Volume"
            FROM spot.valuts
            WHERE user_id = $1
            AND asset_id = $2
            "#,
            user_id,
            asset_id
        )
        .fetch_one(&self.database)
        .await
    }

    pub async fn create_notify_trigger_for_user(&self, user_id: Uuid) -> Result<NotifyTrigger> {
        let trigger_name = trigger_name(
            "spot_valuts_notify_trigger_for_user",
            vec![Bytes::from(user_id.as_bytes().to_owned().to_vec())],
        );
        sqlx::query!(
            r#"
            SELECT create_spot_valuts_notify_trigger_for_user($1, $2)
            "#,
            trigger_name,
            user_id
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
                SELECT drop_spot_valuts_notify_trigger_for_user($1)
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

    pub async fn subscribe_for_user(&self, user_id: Uuid) -> Result<SubscribeStream<Valut>> {
        let mut listener = PgListener::connect_with(&self.database).await?;
        let notify_trigger = self.create_notify_trigger_for_user(user_id).await?;
        listener.listen(&notify_trigger.channel_name).await?;

        let subscribe_stream = listener.into_stream().map(|element| {
            element.and_then(|val| {
                serde_json::from_str::<Valut>(val.payload())
                    .map_err(|err| sqlx::Error::from(std::io::Error::from(err)))
            })
        });

        Ok(SubscribeStream::new(
            notify_trigger,
            Box::pin(subscribe_stream),
        ))
    }

    pub async fn get_or_create_for_user_and_asset(
        &self,
        user_id: Uuid,
        asset_id: Uuid,
    ) -> Result<Valut> {
        sqlx::query_as!(
            Valut,
            r#"
            INSERT INTO spot.valuts (user_id, asset_id)
            VALUES ($1, $2)
            ON CONFLICT (user_id, asset_id) DO NOTHING;
            "#,
            user_id,
            asset_id
        )
        .execute(&self.database)
        .await?;

        self.get_for_user_and_asset(user_id, asset_id).await
    }
}

impl TableManager<Valut> for ValutsManager {
    fn get_all(&self) -> Pin<Box<dyn Stream<Item = Result<Valut>> + Send + '_>> {
        sqlx::query_as!(
            Valut,
            r#"
            SELECT
                id,
                user_id,
                asset_id,
                balance as "balance: Volume"
            FROM spot.valuts
            "#
        )
        .fetch(&self.database)
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Valut> {
        sqlx::query_as!(
            Valut,
            r#"
            SELECT
                id,
                user_id,
                asset_id,
                balance as "balance: Volume"
            FROM spot.valuts
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.database)
        .await
    }

    async fn insert(&self, element: Valut) -> Result<PgQueryResult> {
        let balance: BigDecimal = element.balance.into();
        sqlx::query!(
            r#"
            INSERT INTO
                spot.valuts
                (id, user_id, asset_id, balance)
            VALUES
                ($1, $2, $3, $4)
            "#,
            element.id,
            element.user_id,
            element.asset_id,
            balance
        )
        .execute(&self.database)
        .await
    }

    async fn update(&self, element: Valut) -> Result<PgQueryResult> {
        let balance: BigDecimal = element.balance.into();
        sqlx::query!(
            r#"
            UPDATE 
                spot.valuts 
            SET
                user_id = $2,
                asset_id = $3,
                balance = $4
            WHERE
                id = $1
            "#,
            element.id,
            element.user_id,
            element.asset_id,
            balance
        )
        .execute(&self.database)
        .await
    }

    async fn delete(&self, element: Valut) -> Result<PgQueryResult> {
        sqlx::query!(
            r#"
            DELETE FROM 
                spot.valuts 
            WHERE
                id = $1
            "#,
            element.id,
        )
        .execute(&self.database)
        .await
    }
}
