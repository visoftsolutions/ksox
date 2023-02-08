use database::{
    managers::spot::{assets::AssetsManager, orders::OrdersManager},
    projections::spot::{
        order::{Order, Status},
        trade::Trade,
    },
    sqlx::{
        types::{chrono::Utc, Uuid},
        Pool, Postgres,
    },
    traits::manager::Manager,
};
use futures::StreamExt;
use num_bigint::BigInt;

use self::models::{MatchingEngineRequest, MatchingEngineResponse};
pub mod models;

pub struct MatchingEngine {
    orders_manager: OrdersManager,
    assets_manager: AssetsManager,
}

impl MatchingEngine {
    pub fn new(database: Pool<Postgres>) -> MatchingEngine {
        MatchingEngine {
            orders_manager: OrdersManager::new(database.clone()),
            assets_manager: AssetsManager::new(database),
        }
    }

    pub async fn execute_request(
        &self,
        request: MatchingEngineRequest,
    ) -> Result<MatchingEngineResponse, anyhow::Error> {
        let mut response = MatchingEngineResponse::new();

        let taker_base_asset_id = request.base_asset_id;
        let taker_quote_asset_id = request.quote_asset_id;
        let taker_base_asset = self.assets_manager.get_by_id(taker_base_asset_id).await?;
        let maker_base_asset = self.assets_manager.get_by_id(taker_quote_asset_id).await?;

        let mut taker_quote_asset_volume_left = request.quote_asset_volume.to_owned();
        let mut maker_orders = self.orders_manager.get_ordered_asc_less(
            // base switches with quote to give opposite orderbook
            taker_quote_asset_id,
            taker_base_asset_id,
            // base switches with quote to give inverse of price
            request.quote_asset_volume.to_owned(),
            request.base_asset_volume.to_owned(),
        );

        // buy as much base asset volume as passible with given quote asset volume
        while let Some(maker_order) = maker_orders.next().await && taker_quote_asset_volume_left > BigInt::from(0).into() {
            let maker_order = maker_order?;
            let (mut taker_base_asset_volume_given, taker_quote_asset_volume_taken) =
                if taker_quote_asset_volume_left >= maker_order.base_asset_volume {
                    // eat whole maker_order
                    (
                        maker_order.quote_asset_volume,
                        maker_order.base_asset_volume,
                    )
                } else {
                    // eat maker_order partially
                    // round strategy floor (taker in unfavorable position)
                    (
                        (taker_quote_asset_volume_left.to_owned() * maker_order.base_asset_volume)
                            .checked_div(&maker_order.quote_asset_volume)
                            .ok_or(anyhow::format_err!("division by zero"))?
                            .into(),
                        taker_quote_asset_volume_left.to_owned(),
                    )
                };

            taker_quote_asset_volume_left -= taker_quote_asset_volume_taken.to_owned();

            let (mut maker_base_asset_volume_given, maker_quote_asset_volume_taken) = (
                taker_quote_asset_volume_taken.to_owned(),
                taker_base_asset_volume_given.to_owned(),
            );

            taker_base_asset_volume_given = (taker_base_asset_volume_given
                * (taker_base_asset.to_owned().taker_fee_denum
                    - taker_base_asset.to_owned().taker_fee_num))
                .checked_div(&taker_base_asset.to_owned().taker_fee_denum)
                .ok_or(anyhow::format_err!("division by zero"))?
                .into();
            maker_base_asset_volume_given = (maker_base_asset_volume_given
                * (maker_base_asset.to_owned().maker_fee_denum
                    - maker_base_asset.to_owned().maker_fee_num))
                .checked_div(&taker_base_asset.to_owned().taker_fee_denum)
                .ok_or(anyhow::format_err!("division by zero"))?
                .into();

            response.trades.push(Trade {
                id: Uuid::new_v4(),
                created_at: Utc::now(),
                taker_id: request.user_id,
                order_id: maker_order.id,
                taker_quote_volume: taker_quote_asset_volume_taken,
                taker_base_volume: taker_base_asset_volume_given,
                maker_quote_volume: maker_quote_asset_volume_taken,
                maker_base_volume: maker_base_asset_volume_given,
            });
        }

        if taker_quote_asset_volume_left > BigInt::from(0).into() {
            response.orders.push(Order {
                id: Uuid::new_v4(),
                created_at: Utc::now(),
                user_id: request.user_id,
                status: Status::Active,
                quote_asset_id: taker_quote_asset_id,
                base_asset_id: taker_base_asset_id,
                quote_asset_volume: taker_quote_asset_volume_left.to_owned(),
                base_asset_volume: (taker_quote_asset_volume_left * request.base_asset_volume)
                    .checked_div(&request.quote_asset_volume)
                    .ok_or(anyhow::format_err!("division by zero"))?
                    .into(),
            });
        }
        Ok(response)
    }
}
