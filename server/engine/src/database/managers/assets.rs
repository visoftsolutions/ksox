use fraction::Fraction;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

use crate::database::projections::asset::Asset;

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
            FROM assets
            WHERE assets.id = $1
            "#,
            id
        )
        .fetch_optional(pool.as_mut())
        .await
    }
}
