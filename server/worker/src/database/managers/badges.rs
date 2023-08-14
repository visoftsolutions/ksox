use std::pin::Pin;

use sqlx::postgres::PgPool;
use tokio_stream::Stream;
use uuid::Uuid;

use super::notifications::{
    NotificationManagerOutput, NotificationManagerPredicateInput, NotificationManagerSubscriber,
};
use crate::database::projections::badge::{Badge, BadgeName};

#[derive(Debug, Clone)]
pub struct BadgesManager {
    database: PgPool,
}

impl BadgesManager {
    pub fn new(database: PgPool) -> Self {
        Self { database }
    }

    pub async fn get_modified(
        &self,
        last_modification_at: chrono::DateTime<chrono::Utc>,
    ) -> sqlx::Result<Vec<Badge>> {
        sqlx::query_as!(
            Badge,
            r#"
            SELECT
                engagement.badges.id,
                engagement.badges.created_at,
                engagement.badges.last_modification_at,
                engagement.badges.user_id,
                engagement.badges.badge_name as "badge_name: BadgeName"
            FROM engagement.badges
            WHERE engagement.badges.last_modification_at > $1
            ORDER BY engagement.badges.last_modification_at ASC
            "#,
            last_modification_at
        )
        .fetch_all(&self.database)
        .await
    }

    pub fn get_for_user_id(
        &self,
        user_id: Uuid,
    ) -> Pin<Box<dyn Stream<Item = sqlx::Result<Badge>> + Send + '_>> {
        sqlx::query_as!(
            Badge,
            r#"
            SELECT
                engagement.badges.id,
                engagement.badges.created_at,
                engagement.badges.last_modification_at,
                engagement.badges.user_id,
                engagement.badges.badge_name as "badge_name: BadgeName"
            FROM engagement.badges
            WHERE engagement.badges.user_id = $1
            "#,
            user_id
        )
        .fetch(&self.database)
    }
}

#[derive(Debug, Clone)]
pub struct BadgesNotificationManager {
    notification_manager_subscriber: NotificationManagerSubscriber,
}
impl BadgesNotificationManager {
    pub fn new(notification_manager_subscriber: NotificationManagerSubscriber) -> Self {
        Self {
            notification_manager_subscriber,
        }
    }

    pub async fn subscribe_to_user(
        &self,
        user_id: Uuid,
    ) -> sqlx::Result<Pin<Box<dyn Stream<Item = Vec<Badge>> + Send>>> {
        let p = predicates::function::function(move |input: &NotificationManagerPredicateInput| {
            match input {
                NotificationManagerPredicateInput::EngagementBadges(badge) => {
                    badge.user_id == user_id
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
                    if let NotificationManagerOutput::EngagementBadges(badge) = notification {
                        yield badge;
                    }
                }
            };
            Ok(Box::pin(stream))
        } else {
            Err(sqlx::Error::RowNotFound)
        }
    }
}
