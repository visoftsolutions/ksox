use std::collections::HashSet;

use chrono::{DateTime, Utc};
use fraction::Fraction;
use sqlx::PgPool;
use strum::IntoEnumIterator;
use uuid::Uuid;

use super::Count;
use crate::database::projections::{
    badge::{BadgeName, ValutBadge},
    valut::Valut,
};

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
            FROM spot.valuts
            WHERE last_modification_at > $1
            ORDER BY last_modification_at ASC
            "#,
            last_modification_at,
        )
        .fetch_all(&self.database)
        .await
    }

    pub async fn get_num_of_nonzero_for_user(&self, user_id: Uuid) -> sqlx::Result<i64> {
        let result: Count = sqlx::query_as!(
            Count,
            r#"
            SELECT COALESCE(COUNT(*), 0) as count
            FROM spot.valuts
            WHERE spot.valuts.user_id = $1 AND spot.valuts.balance > (0,1)::fraction
            "#,
            user_id
        )
        .fetch_one(&self.database)
        .await?;
        Ok(result.count.unwrap_or_default())
    }

    pub async fn eval_badges(
        &self,
        user_id: Uuid,
        current_badges: HashSet<BadgeName>,
    ) -> sqlx::Result<HashSet<BadgeName>> {
        let mut potential_badges: HashSet<BadgeName> = HashSet::new();
        let non_zero_valuts = self.get_num_of_nonzero_for_user(user_id).await?;

        for variant in ValutBadge::iter() {
            if non_zero_valuts >= variant.clone() as i64 {
                potential_badges.insert(BadgeName::ValutBadge(variant));
            }
        }

        Ok(potential_badges
            .difference(&current_badges)
            .cloned()
            .collect())
    }
}
