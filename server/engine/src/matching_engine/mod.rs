use database::{
    managers::spot::orders::OrdersManager,
    sqlx::{types::BigDecimal, Pool, Postgres},
};
use futures::StreamExt;

use self::models::execute::{MatchingEngineRequest, MatchingEngineResponse};

pub mod models;

pub struct MatchingEngine {
    orders_manager: OrdersManager,
}

impl MatchingEngine {
    pub fn new(database: Pool<Postgres>) -> MatchingEngine {
        MatchingEngine {
            orders_manager: OrdersManager::new(database),
        }
    }

    pub async fn execute_request(
        &self,
        request: MatchingEngineRequest,
    ) -> Result<MatchingEngineResponse, anyhow::Error> {
        let response = MatchingEngineResponse::new();

        // meaningfull definitions
        let taker_id = request.user_id;
        let taker_base_asset_id = request.base_asset_id;
        let taker_quote_asset_id = request.quote_asset_id;
        let taker_base_asset_volume_left = request.base_asset_volume.clone();
        let taker_quote_asset_volume_left = request.quote_asset_volume.clone();

        // potential maker orders
        let mut maker_orders = self.orders_manager.get_ordered_asc_less(
            // base switches with quote to give opposite orderbook
            taker_quote_asset_id,
            taker_base_asset_id,
            // base switches with quote to give inverse of price
            request.quote_asset_volume,
            request.base_asset_volume,
        );

        while let Some(Ok(maker_order)) = maker_orders.next().await && taker_quote_asset_volume_left > BigDecimal::from(0) {
            todo!()
        }

        todo!()
    }
}
