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
        // TODO implement transaction to revert changes in db when error occurs
        if request.quote_asset_volume <= Volume::from(0)
            || request.base_asset_volume <= Volume::from(0)
        {
            return Err(MatchingEngineError::VolumeIsZero);
        }

        let taker_base_asset = self.assets_manager.get_by_id(request.base_asset_id).await?;
        let maker_base_asset = self
            .assets_manager
            .get_by_id(request.quote_asset_id)
            .await?;

        // subtract from request user valut
        let mut taker_quote_asset_valut = self
            .valuts_manager
            .get_by_user_id_and_asset_id(request.user_id, request.quote_asset_id)
            .await?;
        taker_quote_asset_valut.balance -= request.quote_asset_volume.to_owned();
        self.valuts_manager.update(taker_quote_asset_valut).await?;

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

        // apply order
        if let Some(mut order) = matching.order {
            println!("{:?}", order);
            if !(order.fillable()) {
                order.cancel(&self.valuts_manager).await?;
            }
            self.orders_manager.insert(order).await?;
        }

        // apply trades
        for trade in matching.trades.into_iter() {
            let mut maker_order = self.orders_manager.get_by_id(trade.order_id).await?;

            let mut taker_base_asset_valut = self
                .valuts_manager
                .get_or_create(trade.taker_id, maker_order.quote_asset_id)
                .await?;
            let mut maker_base_asset_valut = self
                .valuts_manager
                .get_or_create(maker_order.user_id, maker_order.base_asset_id)
                .await?;

            // apply changes
            taker_base_asset_valut.balance += trade.taker_base_volume.to_owned();
            maker_base_asset_valut.balance += trade.maker_base_volume.to_owned();
            maker_order.quote_asset_volume_left -= trade.maker_quote_volume.to_owned();

            if !(maker_order.fillable()) {
                maker_order.cancel(&self.valuts_manager).await?;
            }

            // save changes
            self.orders_manager.update(maker_order).await?;
            self.valuts_manager.update(taker_base_asset_valut).await?;
            self.valuts_manager.update(maker_base_asset_valut).await?;
            self.trades_manager.insert(trade).await?;
        }

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
        /*  matching engine axioms:
         *  maker strategy: buy required base asset volume for minimal amount of quote asset
         *  taker strategy: buy as much base asset volume as passible with given quote asset volume
         *  if any asset_volume is zero it is considered unfillable
         */
        println!("{:#?}", taker_fee);
        println!("{:#?}", maker_fee);
        if request_quote_asset_volume <= Volume::from(0)
            || request_base_asset_volume <= Volume::from(0)
        {
            return Err(MatchingEngineError::VolumeIsZero);
        }

        let mut response = MatchingEngineResponse::new();
        let mut taker_quote_asset_volume_left = request_quote_asset_volume.to_owned();

        // && taker_quote_asset_volume_left > Volume::from(0)
        while let Some(maker_order) = available_maker_orders.next().await && taker_quote_asset_volume_left > Volume::from(0) {
            let maker_order = maker_order?;
            let maker_order_base_asset_volume_left = maker_order.base_asset_volume_left_ceil();

            if maker_order.quote_asset_volume_left.to_owned()
                * request_quote_asset_volume.to_owned()
                < maker_order_base_asset_volume_left.to_owned()
                    * request_base_asset_volume.to_owned()
                || request_quote_asset_id != maker_order.base_asset_id
                || request_base_asset_id != maker_order.quote_asset_id
                || request_quote_asset_volume <= Volume::from(0)
                || request_base_asset_volume <= Volume::from(0)
                || !maker_order.is_active
            {
                // reject maker_order price too high or ids invalid or volume less or equal then zero
                continue;
            }

            let (taker_base_asset_volume_given, taker_quote_asset_volume_taken) =
                if taker_quote_asset_volume_left >= maker_order_base_asset_volume_left {
                    // eat whole maker_order
                    (
                        maker_order.quote_asset_volume_left.to_owned(),
                        maker_order_base_asset_volume_left,
                    )
                } else {
                    // eat maker_order partially
                    (
                        (taker_quote_asset_volume_left.to_owned() * maker_order.quote_asset_volume.to_owned())
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
                    - (maker_base_asset_volume_given * maker_order.maker_fee()?),
            });
        }

        if taker_quote_asset_volume_left > Volume::from(0) {
            response.order = Some(Order {
                id: Uuid::new_v4(),
                created_at: Utc::now(),
                user_id: request_user_id,
                is_active: true,
                quote_asset_id: request_quote_asset_id,
                base_asset_id: request_base_asset_id,
                quote_asset_volume: request_quote_asset_volume,
                base_asset_volume: request_base_asset_volume,
                quote_asset_volume_left: taker_quote_asset_volume_left,
                maker_fee_num: maker_fee.numerator.into(),
                maker_fee_denum: maker_fee.denominator.into(),
            });
        }
        Ok(response)
    }
}
