use std::collections::HashSet;

use fraction::Fraction;
use sqlx::{postgres::PgPool, Result};
use strum::IntoEnumIterator;
use uuid::Uuid;

use super::Count;
use crate::database::projections::{
    badge::{BadgeName, TransferBadge},
    transfer::Transfer,
};

#[derive(Debug, Clone)]
pub struct TransfersManager {
    database: PgPool,
}

impl TransfersManager {
    pub fn new(database: PgPool) -> Self {
        Self { database }
    }

    pub async fn get_modified(
        &self,
        last_modification_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<Vec<Transfer>> {
        sqlx::query_as!(
            Transfer,
            r#"
            SELECT
                transfers.id,
                transfers.created_at,
                transfers.last_modification_at,
                transfers.from_valut_id,
                transfers.to_valut_id,
                transfers.asset_id,
                transfers.amount as "amount: Fraction"
            FROM transfers
            WHERE last_modification_at > $1
            ORDER BY last_modification_at ASC
            "#,
            last_modification_at
        )
        .fetch_all(&self.database)
        .await
    }

    pub async fn get_num_maker_transfers_for_user(&self, user_id: Uuid) -> sqlx::Result<i64> {
        let result: Count = sqlx::query_as!(
            Count,
            r#"
            SELECT COALESCE(COUNT(*), 0) as count
            FROM transfers
            WHERE transfers.from_valut_id = $1
            "#,
            user_id
        )
        .fetch_one(&self.database)
        .await?;
        Ok(result.count.unwrap_or_default())
    }

    pub async fn get_num_taker_transfers_for_user(&self, user_id: Uuid) -> sqlx::Result<i64> {
        let result: Count = sqlx::query_as!(
            Count,
            r#"
            SELECT COALESCE(COUNT(*), 0) as count
            FROM transfers
            WHERE transfers.to_valut_id = $1
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
        let maker_transfers = self.get_num_maker_transfers_for_user(user_id).await?;
        let taker_transfers = self.get_num_taker_transfers_for_user(user_id).await?;
        let total_transfers = maker_transfers + taker_transfers;

        for variant in TransferBadge::iter() {
            if total_transfers >= variant.clone() as i64 {
                potential_badges.insert(BadgeName::TransferBadge(variant));
            }
        }

        Ok(potential_badges
            .difference(&current_badges)
            .cloned()
            .collect())
    }
}
