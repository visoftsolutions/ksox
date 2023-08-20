use fraction::Fraction;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

use crate::database::projections::asset::Asset;

#[derive(Debug)]
pub struct AssetsManager {}

impl AssetsManager {
    pub async fn get_by_id<'t>(
        t: &'t mut Transaction<'_, Postgres>,
        id: &Uuid,
    ) -> sqlx::Result<Option<Asset>> {
        sqlx::query_as!(
            Asset,
            r#"
            SELECT
                id,
                decimals as "decimals: Fraction",
                maker_fee as "maker_fee: Fraction",
                taker_fee as "taker_fee: Fraction",
                transfer_fee as "transfer_fee: Fraction"
            FROM assets
            WHERE assets.id = $1
            "#,
            id
        )
        .fetch_optional(t.as_mut())
        .await
    }
}
