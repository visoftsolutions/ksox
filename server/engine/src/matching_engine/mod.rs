use std::pin::Pin;

use database::{
    managers::spot::{
        assets::AssetsManager, orders::OrdersManager, trades::TradesManager, valuts::ValutsManager,
    },
    projections::spot::{order::Order, trade::Trade},
    sqlx::{
        types::{chrono::Utc, Uuid},
        Error, Pool, Postgres,
    },
    traits::table_manager::TableManager,
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
#[cfg(test)]
mod tests;

pub struct MatchingEngine {
    orders_manager: OrdersManager,
    assets_manager: AssetsManager,
    trades_manager: TradesManager,
    valuts_manager: ValutsManager,
}

impl MatchingEngine {
    pub fn new(database: Pool<Postgres>) -> MatchingEngine {
        MatchingEngine {
            orders_manager: OrdersManager::new(database.clone()),
            assets_manager: AssetsManager::new(database.clone()),
            trades_manager: TradesManager::new(database.clone()),
            valuts_manager: ValutsManager::new(database),
        }
    }

    pub async fn execute_request(
        &self,
        request: MatchingEngineRequest,
    ) -> Result<(), MatchingEngineError> {
        // TODO implement transaction to revert changes in db when errors
        let taker_base_asset = self.assets_manager.get_by_id(request.base_asset_id).await?;
        let maker_base_asset = self
            .assets_manager
            .get_by_id(request.quote_asset_id)
            .await?;

        let mut taker_quote_asset_valut = self
            .valuts_manager
            .get_or_create(request.user_id, request.quote_asset_id)
            .await?;
        taker_quote_asset_valut.balance -= request.quote_asset_volume.to_owned();
        if taker_quote_asset_valut.balance >= Volume::from(BigInt::from(0)) {
            self.valuts_manager.update(taker_quote_asset_valut).await?;
        } else {
            return Err(MatchingEngineError::InsufficientBalance);
        }

        let maker_orders = self.orders_manager.get_ordered_asc_less(
            // base switches with quote to give opposite orderbook
            request.quote_asset_id,
            request.base_asset_id,
            // base switches with quote to give inverse of price
            request.quote_asset_volume.to_owned(),
            request.base_asset_volume.to_owned(),
        );

        let matching = Self::matching_loop(
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
        .await?;

        // apply changes to db

        // apply trades first

        for trade in matching.trades.into_iter() {
            let maker_order = self.orders_manager.get_by_id(trade.order_id).await?;
            let maker_id = maker_order.user_id;
            let taker_id = trade.taker_id;
            let taker_quote_asset_id = maker_order.base_asset_id;
            let taker_base_asset_id = maker_order.quote_asset_id;
            let maker_quote_asset_id = maker_order.quote_asset_id;
            let maker_base_asset_id = maker_order.base_asset_id;

            let mut taker_base_asset_valut = self
                .valuts_manager
                .get_or_create(taker_id, taker_base_asset_id)
                .await?;
            let mut maker_base_asset_valut = self
                .valuts_manager
                .get_or_create(maker_id, maker_base_asset_id)
                .await?;

            // apply changes
            taker_base_asset_valut.balance += trade.taker_base_volume;
            maker_base_asset_valut.balance += trade.maker_base_volume;

            // substract from maker_order
            let maker_order_quote_asset_volume_left =
                maker_order.quote_asset_volume - trade.maker_quote_volume;
        }

        // apply orders then

        for order in matching.orders.into_iter() {}

        Ok(())
    }

    pub async fn matching_loop(
        request_user_id: Uuid,
        request_quote_asset_id: Uuid,
        request_base_asset_id: Uuid,
        request_quote_asset_volume: Volume,
        request_base_asset_volume: Volume,
        mut available_maker_orders: Pin<Box<dyn Stream<Item = Result<Order, Error>> + Send + '_>>,
        taker_fee: Fraction,
        maker_fee: Fraction,
    ) -> Result<MatchingEngineResponse, MatchingEngineError> {
        if request_quote_asset_volume <= Volume::from(BigInt::from(0))
            || request_base_asset_volume <= Volume::from(BigInt::from(0))
        {
            return Err(MatchingEngineError::VolumeIsZero);
        }

        let mut response = MatchingEngineResponse::new();
        let mut taker_quote_asset_volume_left = request_quote_asset_volume.to_owned();

        // maker strategy: buy required base asset volume for minimal amount of quote asset
        // taker strategy: buy as much base asset volume as passible with given quote asset volume
        // && taker_quote_asset_volume_left > BigInt::from(0).into()
        while let Some(maker_order) = available_maker_orders.next().await && taker_quote_asset_volume_left > BigInt::from(0).into() {
            let maker_order = maker_order?;

            let maker_order_base_asset_volume_left =
                (maker_order.quote_asset_volume_left.to_owned()
                    * maker_order.base_asset_volume.to_owned()
                    + maker_order.quote_asset_volume.to_owned()
                    - Volume::from(BigInt::from(1)))
                    / maker_order.quote_asset_volume.to_owned();

            if maker_order.quote_asset_volume_left.to_owned()
                * request_quote_asset_volume.to_owned()
                < maker_order_base_asset_volume_left.to_owned()
                    * request_base_asset_volume.to_owned()
                || request_quote_asset_id != maker_order.base_asset_id
                || request_base_asset_id != maker_order.quote_asset_id
                || request_quote_asset_volume <= Volume::from(BigInt::from(0))
                || request_base_asset_volume <= Volume::from(BigInt::from(0))
                || !maker_order.is_active
            {
                // reject maker_order price too high or ids invalid or volume less or equal then zero
                continue;
            }

            let (taker_base_asset_volume_given, taker_quote_asset_volume_taken) =
                if taker_quote_asset_volume_left >= maker_order_base_asset_volume_left {
                    // eat whole maker_order
                    (
                        maker_order.quote_asset_volume_left,
                        maker_order_base_asset_volume_left,
                    )
                } else {
                    // eat maker_order partially
                    (
                        (taker_quote_asset_volume_left.to_owned() * maker_order.quote_asset_volume)
                            .checked_div(&maker_order.base_asset_volume)
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
                taker_base_volume: taker_base_asset_volume_given.to_owned()
                    - (taker_base_asset_volume_given * taker_fee.to_owned()),
                maker_quote_volume: maker_quote_asset_volume_taken,
                maker_base_volume: maker_base_asset_volume_given.to_owned()
                    - (maker_base_asset_volume_given * maker_fee.to_owned()),
            });
        }

        if taker_quote_asset_volume_left > BigInt::from(0).into() {
            response.orders.push(Order {
                id: Uuid::new_v4(),
                created_at: Utc::now(),
                user_id: request_user_id,
                is_active: true,
                quote_asset_id: request_quote_asset_id,
                base_asset_id: request_base_asset_id,
                quote_asset_volume: request_quote_asset_volume,
                base_asset_volume: request_base_asset_volume,
                quote_asset_volume_left: taker_quote_asset_volume_left,
            });
        }
        Ok(response)
    }
}
