use std::pin::Pin;

use bigdecimal::BigDecimal;
use futures::Stream;
use sqlx::{
    postgres::{PgPool, PgQueryResult},
    types::Uuid,
    Result,
};

use crate::{projections::spot::asset::Asset, traits::manager::Manager, types::Volume};

#[derive(Debug, Clone)]
pub struct AssetsManager {
    database: PgPool,
}

impl AssetsManager {
    pub fn new(database: PgPool) -> Self {
        AssetsManager { database }
    }
}

impl Manager<Asset> for AssetsManager {
    fn get_all(&self) -> Pin<Box<dyn Stream<Item = Result<Asset>> + Send + '_>> {
        sqlx::query_as!(
            Asset,
            r#"
            SELECT
                id,
                created_at,
                name,
                symbol,
                maker_fee_num as "maker_fee_num: Volume",
                maker_fee_denum as "maker_fee_denum: Volume",
                taker_fee_num as "taker_fee_num: Volume",
                taker_fee_denum as "taker_fee_denum: Volume"
            FROM spot.assets
            "#
        )
        .fetch(&self.database)
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Asset> {
        sqlx::query_as!(
            Asset,
            r#"
            SELECT
                id,
                created_at,
                name,
                symbol,
                maker_fee_num as "maker_fee_num: Volume",
                maker_fee_denum as "maker_fee_denum: Volume",
                taker_fee_num as "taker_fee_num: Volume",
                taker_fee_denum as "taker_fee_denum: Volume"
            FROM spot.assets
            WHERE spot.assets.id = $1
            "#,
            id
        )
        .fetch_one(&self.database)
        .await
    }

    async fn insert(&self, element: Asset) -> Result<PgQueryResult> {
        let maker_fee_num: BigDecimal = element.maker_fee_num.into();
        let maker_fee_denum: BigDecimal = element.maker_fee_denum.into();
        let taker_fee_num: BigDecimal = element.taker_fee_num.into();
        let taker_fee_denum: BigDecimal = element.taker_fee_denum.into();
        sqlx::query!(
            r#"
            INSERT INTO 
                spot.assets 
                (id, created_at, name, symbol, maker_fee_num, maker_fee_denum, taker_fee_num, taker_fee_denum)
            VALUES
                ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            element.id,
            element.created_at,
            element.name,
            element.symbol,
            maker_fee_num,
            maker_fee_denum,
            taker_fee_num,
            taker_fee_denum
        )
        .execute(&self.database)
        .await
    }

    async fn update(&self, element: Asset) -> Result<PgQueryResult> {
        let maker_fee_num: BigDecimal = element.maker_fee_num.into();
        let maker_fee_denum: BigDecimal = element.maker_fee_denum.into();
        let taker_fee_num: BigDecimal = element.taker_fee_num.into();
        let taker_fee_denum: BigDecimal = element.taker_fee_denum.into();
        sqlx::query!(
            r#"
            UPDATE 
                spot.assets 
            SET
                created_at = $2,
                name = $3,
                symbol = $4,
                maker_fee_num = $5,
                maker_fee_denum = $6,
                taker_fee_num = $7,
                taker_fee_denum = $8
            WHERE
                id = $1
            "#,
            element.id,
            element.created_at,
            element.name,
            element.symbol,
            maker_fee_num,
            maker_fee_denum,
            taker_fee_num,
            taker_fee_denum
        )
        .execute(&self.database)
        .await
    }

    async fn delete(&self, element: Asset) -> Result<PgQueryResult> {
        sqlx::query!(
            r#"
            DELETE FROM 
                spot.assets 
            WHERE
                id = $1
            "#,
            element.id,
        )
        .execute(&self.database)
        .await
    }
}
