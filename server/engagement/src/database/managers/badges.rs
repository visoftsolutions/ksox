use sqlx::postgres::PgPool;
use uuid::Uuid;

use crate::database::projections::badge::Badge;

#[derive(Debug, Clone)]
pub struct BadgesManager {
    database: PgPool,
}

impl BadgesManager {
    pub fn new(database: PgPool) -> Self {
        Self { database }
    }

    pub async fn badge_id_to_badge(&self, badge_id: String) -> sqlx::Result<Badge> {
        sqlx::query_as!(
            Badge,
            r#"
        SELECT
            engagement.badges.id,
            engagement.badges.name as "badge_name: String",
            engagement.badges.family as "badge_family: String",
            engagement.badges.description as "badge_description: String",
            engagement.badges.value,
            engagement.badges.created_at
        FROM
            engagement.badges
        WHERE
            engagmenet.badges.id = $1
        "#,
            badge_id
        )
        .fetch_one(&self.database)
        .await
    }

    pub async fn get_all_badges(&self) -> sqlx::Result<Vec<Badge>> {
        sqlx::query_as!(
            Badge,
            r#"
            SELECT
                engagement.badges.id,
                engagement.badges.name as "badge_name: String",
                engagement.badges.family as "badge_family: String",
                engagement.badges.description as "badge_description: String",
                engagement.badges.value,
                engagement.badges.created_at
            FROM
                engagement.badges
            "#
        )
        .fetch_all(&self.database)
        .await
    }

    pub async fn get_badges_for_family(&self, family: String) -> sqlx::Result<Vec<Badge>> {
        sqlx::query_as!(
            Badge,
            r#"
            SELECT 
                engagement.badges.id,
                engagement.badges.name as "badge_name: String",
                engagement.badges.family as "badge_family: String",
                engagement.badges.description as "badge_description: String",
                engagement.badges.value,
                engagement.badges.created_at
            FROM
                engagement.badges
            WHERE
                engagement.badges.family = $1
            "#,
            family
        )
        .fetch_all(&self.database)
        .await
    }

    pub async fn get_badge_id(&self, family: String, value: u64) -> sqlx::Result<Uuid> {
        sqlx::query_as!(
            Uuid,
            r#"
            SELECT 
                engagement.badges.id
            FROM
                engagement.badges
            WHERE
                engagement.badges.family = $1 AND engagement.badges.value = $2
        "#,
            family,
            value
        )
        .fetch_one(&self.database)
        .await
    }
}
