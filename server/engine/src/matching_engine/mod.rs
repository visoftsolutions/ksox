use database::{
    managers::spot::{
        assets::AssetsManager, orders::OrdersManager, trades::TradesManager, valuts::ValutsManager,
    },
    projections::spot::{order::Order, trade::Trade},
    sqlx::{
        types::{chrono::Utc, Uuid},
        Pool, Postgres, postgres::PgAdvisoryLock,
    },
    traits::table_manager::TableManager,
    types::Volume,
};
use futures::StreamExt;

use self::models::{
    MatchingEngineData, MatchingEngineError, MatchingEngineRequest, MatchingEngineResponse,
};

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
    pub fn new(database: Pool<Postgres>, lock: PgAdvisoryLock) -> MatchingEngine {
        MatchingEngine {
            orders_manager: OrdersManager::new(database.clone()),
            assets_manager: AssetsManager::new(database.clone()),
            trades_manager: TradesManager::new(database.clone()),
            valuts_manager: ValutsManager::new(database, lock),
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
            .get_for_user_asset(request.user_id, request.quote_asset_id)
            .await?;
        taker_quote_asset_valut.balance -= request.quote_asset_volume.to_owned();
        if taker_quote_asset_valut.balance >= Volume::from(0) {
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

        let matching_loop_input = MatchingEngineData::new(
            request.user_id,
            request.quote_asset_id,
            request.base_asset_id,
            request.quote_asset_volume,
            request.base_asset_volume,
            maker_orders,
            taker_base_asset.taker_fee,
            maker_base_asset.maker_fee,
        );

        let matching = Self::matching_loop(matching_loop_input).await?;

        // apply order
        if let Some(mut order) = matching.order {
            if !order.fillable() {
                order.cancel(&self.valuts_manager).await?;
            }
            self.orders_manager.insert(order).await?;
        }

        // apply trades
        for trade in matching.trades.into_iter() {
            let mut maker_order = self.orders_manager.get_by_id(trade.order_id).await?;

            let mut taker_base_asset_valut = self
                .valuts_manager
                .get_or_create_for_user_asset(trade.taker_id, maker_order.quote_asset_id)
                .await?;
            let mut maker_base_asset_valut = self
                .valuts_manager
                .get_or_create_for_user_asset(maker_order.user_id, maker_order.base_asset_id)
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

    pub async fn cancel_order(&self, order_id: Uuid) -> Result<(), MatchingEngineError> {
        let mut order = self.orders_manager.get_by_id(order_id).await?;
        if !order.is_active {
            return Err(MatchingEngineError::NotActive);
        }
        order.cancel(&self.valuts_manager).await?;
        self.orders_manager.update(order).await?;
        Ok(())
    }

    pub async fn matching_loop(
        mut input: MatchingEngineData<'_>,
    ) -> Result<MatchingEngineResponse, MatchingEngineError> {
        /*  matching engine axioms:
         *  maker strategy: buy required base asset volume for minimal amount of quote asset
         *  taker strategy: buy as much base asset volume as passible with given quote asset volume
         *  if any asset_volume is zero it is considered unfillable
         */
        if input.quote_asset_volume <= Volume::from(0) || input.base_asset_volume <= Volume::from(0)
        {
            return Err(MatchingEngineError::VolumeIsZero);
        }

        let mut response = MatchingEngineResponse::new();
        let mut taker_quote_asset_volume_left = input.quote_asset_volume.to_owned();

        // && taker_quote_asset_volume_left > Volume::from(0)
        while let Some(maker_order) = input.maker_orders.next().await && taker_quote_asset_volume_left > Volume::from(0) {
            let maker_order = maker_order?;
            let maker_order_base_asset_volume_left = maker_order.base_asset_volume_left_ceil();

            if maker_order.quote_asset_volume_left.to_owned() * input.quote_asset_volume.to_owned()
                < maker_order_base_asset_volume_left.to_owned() * input.base_asset_volume.to_owned()
                || input.quote_asset_id != maker_order.base_asset_id
                || input.base_asset_id != maker_order.quote_asset_id
                || input.quote_asset_volume <= Volume::from(0)
                || input.base_asset_volume <= Volume::from(0)
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
                        (taker_quote_asset_volume_left.to_owned()
                            * maker_order.quote_asset_volume.to_owned())
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

            let now = Utc::now();
            response.trades.push(Trade {
                id: Uuid::new_v4(),
                created_at: now,
                last_modification_at: now,
                quote_asset_id: input.quote_asset_id,
                base_asset_id: input.base_asset_id,
                taker_id: input.user_id,
                order_id: maker_order.id,
                taker_quote_volume: taker_quote_asset_volume_taken,
                taker_base_volume: taker_base_asset_volume_given.to_owned()
                    - (taker_base_asset_volume_given * input.taker_fee.to_owned()),
                maker_quote_volume: maker_quote_asset_volume_taken,
                maker_base_volume: maker_base_asset_volume_given.to_owned()
                    - (maker_base_asset_volume_given * maker_order.maker_fee),
            });
        }

        if taker_quote_asset_volume_left > Volume::from(0) {
            let now = Utc::now();
            response.order = Some(Order {
                id: Uuid::new_v4(),
                created_at: now,
                last_modification_at: now,
                user_id: input.user_id,
                is_active: true,
                quote_asset_id: input.quote_asset_id,
                base_asset_id: input.base_asset_id,
                quote_asset_volume: input.quote_asset_volume,
                base_asset_volume: input.base_asset_volume,
                quote_asset_volume_left: taker_quote_asset_volume_left,
                maker_fee: input.maker_fee,
            });
        }
        Ok(response)
    }
}
