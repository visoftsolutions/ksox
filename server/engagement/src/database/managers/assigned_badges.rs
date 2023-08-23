use crate::database::projections::assigned_badge::AssignedBadge;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct AssignedBadgesManager {
    database: PgPool,
}

impl AssignedBadgesManager {
    pub fn new(database: PgPool) -> Self {
        Self { database }
    }

    pub async fn get_badges_id_for_user_id(&self, user_id: Uuid) -> sqlx::Result<Vec<String>> {
        sqlx::query_as!(
            AssignedBadge,
            r#"
            SELECT 
                engagement.assigned_badges.badge_id
            FROM
                engagement.assigned_badges
            WHERE
                engagement.assigned_badges.user_id = $1
        "#,
            user_id
        )
        .fetch_all(&self.database)
        .await
    }

    // TODO
    // pub fn assign_badge(&self) -> sqlx::Result<PgQueryResult> {}

    pub async fn get_modified(
        &self,
        last_modification_at: DateTime<Utc>,
    ) -> sqlx::Result<Vec<AssignedBadge>> {
        sqlx::query_as!(
            AssignedBadge,
            r#"
            SELECT 
                engagement.assigned_badges.id,
                engagement.assigned_badges.user_id,
                engagement.assigned_badges.badge_id,
                engagement.assigned_badges.created_at,
                engagement.assigned_badges.last_modification_at
            FROM 
                engagement.assigned_badges
            WHERE 
                engagement.last_modification_at > $1
            "#,
            last_modification_at
        )
        .fetch_all(&self.database)
        .await
    }
}
