use std::pin::Pin;

use chrono::{DateTime, Utc};
use fraction::Fraction;
use futures::Stream;
use sqlx::{postgres::PgQueryResult, PgPool};
use uuid::Uuid;

use super::notifications::{
    NotificationManagerOutput, NotificationManagerPredicateInput, NotificationManagerSubscriber,
};
use crate::database::projections::valut::Valut;

#[derive(Debug, Clone)]
pub struct ValutsManager {
    database: PgPool,
}

impl ValutsManager {
    pub fn new(database: PgPool) -> Self {
        ValutsManager { database }
    }

    pub async fn get_modified(
        &self,
        last_modification_at: DateTime<Utc>,
    ) -> sqlx::Result<Vec<Valut>> {
        sqlx::query_as!(
            Valut,
            r#"
            SELECT
                id,
                created_at,
                last_modification_at,
                user_id,
                asset_id,
                balance as "balance: Fraction"
            FROM valuts
            WHERE last_modification_at > $1
            ORDER BY last_modification_at ASC
            "#,
            last_modification_at,
        )
        .fetch_all(&self.database)
        .await
    }

    pub async fn get_for_user_asset(&self, user_id: Uuid, asset_id: Uuid) -> sqlx::Result<Valut> {
        sqlx::query_as!(
            Valut,
            r#"
            SELECT
                id,
                created_at,
                last_modification_at,
                user_id,
                asset_id,
                balance as "balance: Fraction"
            FROM valuts
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
    ) -> sqlx::Result<Valut> {
        sqlx::query_as!(
            Valut,
            r#"
            INSERT INTO valuts (last_modification_at, user_id, asset_id, balance)
            VALUES ($1, $2, $3, (0,1))
            ON CONFLICT (user_id, asset_id) DO NOTHING;
            "#,
            chrono::Utc::now(),
            user_id,
            asset_id
        )
        .execute(&self.database)
        .await?;
        self.get_for_user_asset(user_id, asset_id).await
    }

    pub async fn update(&self, element: Valut) -> sqlx::Result<PgQueryResult> {
        sqlx::query!(
            r#"
            UPDATE 
                valuts 
            SET
                last_modification_at = $2,
                user_id = $3,
                asset_id = $4,
                balance = $5::fraction
            WHERE
                id = $1
            "#,
            element.id,
            chrono::Utc::now(),
            element.user_id,
            element.asset_id,
            element.balance.to_tuple_string() as _
        )
        .execute(&self.database)
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
    ) -> sqlx::Result<Pin<Box<dyn Stream<Item = Vec<Valut>> + Send>>> {
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
