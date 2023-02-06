use database::{
    managers::spot::{assets::AssetsManager, orders::OrdersManager},
    projections::spot::{
        order::{Order, Status},
        trade::Trade,
    },
    sqlx::{
        types::{chrono::Utc, BigDecimal, Uuid},
        Pool, Postgres,
    },
    traits::manager::Manager,
};
use futures::StreamExt;

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
        let mut taker_quote_asset_volume_left = request.quote_asset_volume.clone();

        let (_, taker_base_asset_precission) = request.base_asset_volume.as_bigint_and_exponent();
        let (_, maker_base_asset_precission) = request.quote_asset_volume.as_bigint_and_exponent();

        let taker_fee = self
            .assets_manager
            .get_by_id(taker_base_asset_id)
            .await?
            .taker_fee;
        let maker_fee = self
            .assets_manager
            .get_by_id(taker_quote_asset_id)
            .await?
            .maker_fee;

        let mut maker_orders = self.orders_manager.get_ordered_asc_less(
            // base switches with quote to give opposite orderbook
            taker_quote_asset_id,
            taker_base_asset_id,
            // base switches with quote to give inverse of price
            request.quote_asset_volume.clone(),
            request.base_asset_volume.clone(),
        );

        // buy as much base asset volume as passible with given quote asset volume
        while let Some(maker_order) = maker_orders.next().await && BigDecimal::from(0) < taker_quote_asset_volume_left {
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
                        (taker_quote_asset_volume_left.clone() * maker_order.base_asset_volume
                            / maker_order.quote_asset_volume)
                            .round(taker_base_asset_precission),
                        taker_quote_asset_volume_left.clone(),
                    )
                };

            taker_quote_asset_volume_left -= taker_quote_asset_volume_taken.clone();

            let (mut maker_base_asset_volume_given, maker_quote_asset_volume_taken) = (
                taker_quote_asset_volume_taken.clone(), taker_base_asset_volume_given.clone()
            );

            taker_base_asset_volume_given = (taker_base_asset_volume_given * (BigDecimal::from(1) - taker_fee.clone())).round(taker_base_asset_precission);
            maker_base_asset_volume_given = (maker_base_asset_volume_given * (BigDecimal::from(1) - maker_fee.clone())).round(maker_base_asset_precission);

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

        if BigDecimal::from(0) < taker_quote_asset_volume_left {
            response.orders.push(Order {
                id: Uuid::new_v4(),
                created_at: Utc::now(),
                user_id: request.user_id,
                status: Status::Active,
                quote_asset_id: taker_quote_asset_id,
                base_asset_id: taker_base_asset_id,
                quote_asset_volume: taker_quote_asset_volume_left.clone(),
                base_asset_volume: (taker_quote_asset_volume_left * request.base_asset_volume
                    / request.quote_asset_volume)
                    .round(taker_base_asset_precission),
            });
        }
        Ok(response)
    }
}
