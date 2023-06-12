use std::collections::HashSet;

use chrono::{DateTime, Utc};
use fraction::Fraction;
use sqlx::PgPool;
use strum::IntoEnumIterator;
use uuid::Uuid;

use super::Count;
use crate::database::projections::{
    badge::{BadgeName, MakerBadge, TakerBadge, TradeBadge},
    trade::Trade,
};

#[derive(Debug, Clone)]
pub struct TradesManager {
    database: PgPool,
}

impl TradesManager {
    pub fn new(database: PgPool) -> Self {
        TradesManager { database }
    }

    pub async fn get_modified(
        &self,
        last_modification_at: DateTime<Utc>,
    ) -> sqlx::Result<Vec<Trade>> {
        sqlx::query_as!(
            Trade,
            r#"
            SELECT
                spot.trades.id,
                spot.trades.created_at,
                spot.trades.last_modification_at,
                spot.trades.quote_asset_id,
                spot.trades.base_asset_id,
                spot.trades.taker_id,
                spot.trades.taker_presentation,
                spot.orders.maker_id,
                spot.orders.maker_presentation,
                spot.trades.price as "price: Fraction",
                spot.trades.taker_quote_volume as "taker_quote_volume: Fraction",
                spot.trades.taker_base_volume as "taker_base_volume: Fraction",
                spot.trades.maker_quote_volume as "maker_quote_volume: Fraction",
                spot.trades.maker_base_volume as "maker_base_volume: Fraction"
            FROM spot.trades
            JOIN spot.orders ON spot.orders.id = spot.trades.order_id
            WHERE spot.trades.last_modification_at > $1
            ORDER BY spot.trades.last_modification_at ASC
            "#,
            last_modification_at
        )
        .fetch_all(&self.database)
        .await
    }

    pub async fn get_num_maker_trades_for_user(&self, user_id: Uuid) -> sqlx::Result<i64> {
        let result: Count = sqlx::query_as!(
            Count,
            r#"
            SELECT COALESCE(COUNT(*), 0) as count
            FROM spot.trades
            JOIN spot.orders ON spot.orders.id = spot.trades.order_id
            WHERE spot.orders.maker_id = $1
            "#,
            user_id
        )
        .fetch_one(&self.database)
        .await?;
        Ok(result.count.unwrap_or_default())
    }

    pub async fn get_num_taker_trades_for_user(&self, user_id: Uuid) -> sqlx::Result<i64> {
        let result: Count = sqlx::query_as!(
            Count,
            r#"
            SELECT COALESCE(COUNT(*), 0) as count
            FROM spot.trades
            WHERE spot.trades.taker_id = $1
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
        let maker_trades = self.get_num_maker_trades_for_user(user_id).await?;
        let taker_trades = self.get_num_taker_trades_for_user(user_id).await?;
        let total_trades = maker_trades + taker_trades;

        for variant in TradeBadge::iter() {
            if total_trades >= variant.clone() as i64 {
                potential_badges.insert(BadgeName::TradeBadge(variant));
            }
        }

        for variant in MakerBadge::iter() {
            if maker_trades >= variant.clone() as i64 {
                potential_badges.insert(BadgeName::MakerBadge(variant));
            }
        }

        for variant in TakerBadge::iter() {
            if taker_trades >= variant.clone() as i64 {
                potential_badges.insert(BadgeName::TakerBadge(variant));
            }
        }

        Ok(potential_badges
            .difference(&current_badges)
            .cloned()
            .collect())
    }
}
