use std::pin::Pin;

use database::{
    managers::spot::{assets::AssetsManager, orders::OrdersManager},
    projections::spot::{
        order::{Order, Status},
        trade::Trade,
    },
    sqlx::{
        types::{chrono::Utc, Uuid},
        Error, Pool, Postgres,
    },
    traits::manager::Manager,
    types::{Fraction, Volume},
};
use futures::{Stream, StreamExt};
use num_bigint::BigInt;

use self::{
    errors::MatchingEngineError,
    models::{MatchingEngineRequest, MatchingEngineResponse},
};
pub mod errors;
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
    ) -> Result<MatchingEngineResponse, MatchingEngineError> {
        let taker_base_asset = self.assets_manager.get_by_id(request.base_asset_id).await?;
        let maker_base_asset = self
            .assets_manager
            .get_by_id(request.quote_asset_id)
            .await?;

        let maker_orders = self.orders_manager.get_ordered_asc_less(
            // base switches with quote to give opposite orderbook
            request.quote_asset_id,
            request.base_asset_id,
            // base switches with quote to give inverse of price
            request.quote_asset_volume.to_owned(),
            request.base_asset_volume.to_owned(),
        );

        Self::matching_loop(
            request.user_id,
            request.quote_asset_id,
            request.base_asset_id,
            request.quote_asset_volume,
            request.base_asset_volume,
            maker_orders,
            (
                taker_base_asset.taker_fee_num.into(),
                taker_base_asset.taker_fee_denum.into(),
            )
                .try_into()?,
            (
                maker_base_asset.maker_fee_num.into(),
                maker_base_asset.maker_fee_denum.into(),
            )
                .try_into()?,
        )
        .await
    }

    async fn matching_loop(
        request_user_id: Uuid,
        request_quote_asset_id: Uuid,
        request_base_asset_id: Uuid,
        request_quote_asset_volume: Volume,
        request_base_asset_volume: Volume,
        mut available_maker_orders: Pin<Box<dyn Stream<Item = Result<Order, Error>> + Send + '_>>,
        taker_fee: Fraction,
        maker_fee: Fraction,
    ) -> Result<MatchingEngineResponse, MatchingEngineError> {
        let mut response = MatchingEngineResponse::new();
        let mut taker_quote_asset_volume_left = request_quote_asset_volume.to_owned();

        // buy as much base asset volume as passible with given quote asset volume
        //  && taker_quote_asset_volume_left > BigInt::from(0).into()
        while let Some(maker_order) = available_maker_orders.next().await {
            let maker_order = maker_order?;
            let (taker_base_asset_volume_given, taker_quote_asset_volume_taken) =
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
                            .ok_or(MatchingEngineError::DivisionByZero)?
                            .into(),
                        taker_quote_asset_volume_left.to_owned(),
                    )
                };

            taker_quote_asset_volume_left -= taker_quote_asset_volume_taken.to_owned();

            let (maker_base_asset_volume_given, maker_quote_asset_volume_taken) = (
                taker_quote_asset_volume_taken.to_owned(),
                taker_base_asset_volume_given.to_owned(),
            );

            response.trades.push(Trade {
                id: Uuid::new_v4(),
                created_at: Utc::now(),
                taker_id: request_user_id,
                order_id: maker_order.id,
                taker_quote_volume: taker_quote_asset_volume_taken,
                taker_base_volume: taker_base_asset_volume_given * (BigInt::from(1) - taker_fee.to_owned()),
                maker_quote_volume: maker_quote_asset_volume_taken,
                maker_base_volume: maker_base_asset_volume_given * (BigInt::from(1) - maker_fee.to_owned()),
            });
        }

        if taker_quote_asset_volume_left > BigInt::from(0).into() {
            response.orders.push(Order {
                id: Uuid::new_v4(),
                created_at: Utc::now(),
                user_id: request_user_id,
                status: Status::Active,
                quote_asset_id: request_quote_asset_id,
                base_asset_id: request_base_asset_id,
                quote_asset_volume: taker_quote_asset_volume_left.to_owned(),
                base_asset_volume: (taker_quote_asset_volume_left * request_base_asset_volume)
                    .checked_div(&request_quote_asset_volume)
                    .ok_or(MatchingEngineError::DivisionByZero)?
                    .into(),
            });
        }
        Ok(response)
    }
}
