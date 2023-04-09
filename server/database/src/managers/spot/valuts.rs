use std::pin::Pin;

use bigdecimal::BigDecimal;
use futures::Stream;
use sqlx::{
    postgres::{PgAdvisoryLock, PgPool, PgQueryResult},
    types::Uuid,
    Result,
};

use crate::{
    managers::notifications::{
        NotificationManagerOutput, NotificationManagerPredicateInput, NotificationManagerSubscriber,
    },
    projections::spot::valut::Valut,
    traits::{get_modified::GetModified, table_manager::TableManager},
    types::{SubscribeStream, Volume},
};

#[derive(Debug, Clone)]
pub struct ValutsManager {
    database: PgPool,
    lock: PgAdvisoryLock,
}

impl ValutsManager {
    pub fn new(database: PgPool, lock: PgAdvisoryLock) -> Self {
        ValutsManager { database, lock }
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
                created_at,
                last_modification_at,
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

    pub async fn get_for_user_asset(&self, user_id: Uuid, asset_id: Uuid) -> Result<Valut> {
        sqlx::query_as!(
            Valut,
            r#"
            SELECT
                id,
                created_at,
                last_modification_at,
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

    pub async fn get_or_create_for_user_asset(
        &self,
        user_id: Uuid,
        asset_id: Uuid,
    ) -> Result<Valut> {
        let mut transaction = self.database.begin().await?;
        sqlx::query!(
            r#"
            LOCK TABLE spot.valuts IN ACCESS EXCLUSIVE MODE;
            "#
        )
        .execute(&mut transaction)
        .await?;
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
        .execute(&mut transaction)
        .await?;
        transaction.commit().await?;
        self.get_for_user_asset(user_id, asset_id).await
    }
}

impl TableManager<Valut> for ValutsManager {
    fn get_all(&self) -> Pin<Box<dyn Stream<Item = Result<Valut>> + Send + '_>> {
        sqlx::query_as!(
            Valut,
            r#"
            SELECT
                id,
                created_at,
                last_modification_at,
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
                created_at,
                last_modification_at,
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
        let mut transaction = self.database.begin().await?;
        sqlx::query!(
            r#"
            LOCK TABLE spot.valuts IN ACCESS EXCLUSIVE MODE;
            "#
        )
        .execute(&mut transaction)
        .await?;
        let result = sqlx::query!(
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
        .execute(&mut transaction)
        .await?;
        transaction.commit().await?;
        Ok(result)
    }

    async fn update(&self, element: Valut) -> Result<PgQueryResult> {
        let balance: BigDecimal = element.balance.into();
        let mut transaction = self.database.begin().await?;
        sqlx::query!(
            r#"
            LOCK TABLE spot.valuts IN ACCESS EXCLUSIVE MODE;
            "#
        )
        .execute(&mut transaction)
        .await?;
        let result = sqlx::query!(
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
        .execute(&mut transaction)
        .await?;
        transaction.commit().await?;
        Ok(result)
    }

    async fn delete(&self, element: Valut) -> Result<PgQueryResult> {
        let mut transaction = self.database.begin().await?;
        sqlx::query!(
            r#"
            LOCK TABLE spot.valuts IN ACCESS EXCLUSIVE MODE;
            "#
        )
        .execute(&mut transaction)
        .await?;
        let result = sqlx::query!(
            r#"
            DELETE FROM 
                spot.valuts 
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

impl GetModified<Valut> for ValutsManager {
    async fn get_modified(
        &self,
        last_modification_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<Vec<Valut>> {
        sqlx::query_as!(
            Valut,
            r#"
                SELECT
                    id,
                    created_at,
                    last_modification_at,
                    user_id,
                    asset_id,
                    balance as "balance: Volume"
                FROM spot.valuts
                WHERE last_modification_at > $1
                "#,
            last_modification_at
        )
        .fetch_all(&self.database)
        .await
    }
}

#[derive(Debug, Clone)]
pub struct ValutsNotificationManager {
    notification_manager_subscriber: NotificationManagerSubscriber,
}
impl ValutsNotificationManager {
    pub fn new(notification_manager_subscriber: NotificationManagerSubscriber) -> Self {
        Self {
            notification_manager_subscriber,
        }
    }

    pub async fn subscribe_for_user_asset(
        &self,
        user_id: Uuid,
        asset_id: Uuid,
    ) -> Result<SubscribeStream<Vec<Valut>>> {
        let p = predicates::function::function(move |input: &NotificationManagerPredicateInput| {
            match input {
                NotificationManagerPredicateInput::SpotValutsChanged(valut) => {
                    valut.user_id == user_id && valut.asset_id == asset_id
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
                    if let NotificationManagerOutput::SpotValutsChanged(valuts) = notification {
                        yield valuts;
                    }
                }
            };
            Ok(Box::pin(stream))
        } else {
            Err(sqlx::Error::RowNotFound)
        }
    }
}
