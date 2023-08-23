use std::pin::Pin;

use sqlx::postgres::PgPool;
use tokio_stream::Stream;
use uuid::Uuid;

use super::notifications::{
    NotificationManagerOutput, NotificationManagerPredicateInput, NotificationManagerSubscriber,
};
use crate::database::projections::badge::Badge;

#[derive(Debug, Clone)]
pub struct BadgesManager {
    database: PgPool,
}

impl BadgesManager {
    pub fn new(database: PgPool) -> Self {
        Self { database }
    }
    pub fn badge_id_to_badge(&self, badge_id: String) -> sqlx::Result<Badge> {
        sqlx::query_as!(
            Badge,
            r#"
            SELECT 
            "#
        )
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
                NotificationManagerPredicateInput::EngagementBadgesChanged(badge) => {
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
                    if let NotificationManagerOutput::EngagementBadgesChanged(badge) = notification {
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
