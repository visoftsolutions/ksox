use chrono::Utc;
use sqlx::postgres::{PgPool, PgQueryResult};
use uuid::Uuid;

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

    pub async fn get_for_user_id(&self, user_id: Uuid) -> sqlx::Result<Vec<Badge>> {
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
        .fetch_all(&self.database)
        .await
    }

    pub async fn assign_badge(
        &self,
        user_id: Uuid,
        badge_name: BadgeName,
    ) -> sqlx::Result<PgQueryResult> {
        let now = Utc::now();
        sqlx::query!(
            r#"
            INSERT INTO engagement.badges
                (created_at, last_modification_at, user_id, badge_name)
            VALUES
                ($1, $2, $3, $4)
            "#,
            now,
            now,
            user_id,
            badge_name as _
        )
        .execute(&self.database)
        .await
    }
}
