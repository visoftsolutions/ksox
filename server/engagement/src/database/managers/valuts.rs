use std::{cmp::Ordering, collections::HashSet};

use chrono::{DateTime, Utc};
use fraction::Fraction;
use sqlx::PgPool;
use strum::IntoEnumIterator;
use uuid::Uuid;

use crate::database::projections::{
    badge::{BadgeName, ValutBadge},
    valut::Valut,
};
use fraction::num_traits::Zero;
use value::Value;

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
                balance as "balance: Value"
            FROM valuts
            WHERE last_modification_at > $1
            ORDER BY last_modification_at ASC
            "#,
            last_modification_at,
        )
        .fetch_all(&self.database)
        .await
    }

    pub async fn get_for_user(&self, user_id: Uuid) -> sqlx::Result<Vec<Valut>> {
        sqlx::query_as!(
            Valut,
            r#"
            SELECT
                id,
                created_at,
                last_modification_at,
                user_id,
                asset_id,
                balance as "balance: Value"
            FROM valuts
            WHERE valuts.user_id = $1
            "#,
            user_id
        )
        .fetch_all(&self.database)
        .await
    }

    pub async fn get_num_of_nonzero_for_user(&self, user_id: Uuid) -> sqlx::Result<i64> {
        let valuts = self.get_for_user(user_id).await?;
        Ok(valuts.into_iter().fold(0, |acc, e| {
            match e.balance.cmp(&Value::Finite(Fraction::zero())) {
                Ordering::Greater => acc + 1,
                _ => acc,
            }
        }))
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
