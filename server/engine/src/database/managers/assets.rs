use sqlx::{Postgres, Transaction};
use uuid::Uuid;

use crate::{database::Asset, types::Fraction};

#[derive(Debug)]
pub struct AssetsManager {}

impl AssetsManager {
    pub async fn get_by_id<'t, 'p>(
        pool: &'t mut Transaction<'p, Postgres>,
        id: Uuid,
    ) -> sqlx::Result<Option<Asset>> {
        sqlx::query_as!(
            Asset,
            r#"
            SELECT
                id,
                maker_fee as "maker_fee: Fraction",
                taker_fee as "taker_fee: Fraction"
            FROM spot.assets
            WHERE spot.assets.id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await
    }
}
