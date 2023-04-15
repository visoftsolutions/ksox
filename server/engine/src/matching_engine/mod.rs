use std::pin::Pin;

use base::engine_server::Engine;
use num_traits::{Inv, Zero};
use sqlx::{Pool, Postgres};
use tokio_stream::{Stream, StreamExt};
use tonic::{Request, Response, Status};
use uuid::Uuid;

use self::models::{
    CancelRequest, CancelRequestError, CancelResponse, MatchingLoopError, MatchingLoopResponse,
    SubmitRequest, SubmitRequestError, SubmitResponse,
};
use crate::{
    base,
    database::{
        Asset, AssetsManager, OrderGet, OrderInsert, OrdersManager, Trade, TradesManager,
        ValutsManager,
    },
    types::Fraction,
};
pub mod models;
pub struct MatchingEngine {
    accuracy: Fraction,
    orders_manager: OrdersManager,
    assets_manager: AssetsManager,
    trades_manager: TradesManager,
    valuts_manager: ValutsManager,
}

impl MatchingEngine {
    pub fn new(database: Pool<Postgres>, accuracy: Fraction) -> Self {
        Self {
            accuracy,
            orders_manager: OrdersManager::new(database.clone()),
            assets_manager: AssetsManager::new(database.clone()),
            trades_manager: TradesManager::new(database.clone()),
            valuts_manager: ValutsManager::new(database),
        }
    }
}

#[tonic::async_trait]
impl Engine for MatchingEngine {
    async fn submit(
        &self,
        request: Request<base::SubmitRequest>,
    ) -> Result<Response<base::SubmitResponse>, Status> {
        Ok(Response::new(
            self.submit(request.into_inner().try_into()?)
                .await
                .try_into()?,
        ))
    }

    async fn cancel(
        &self,
        request: Request<base::CancelRequest>,
    ) -> Result<Response<base::CancelResponse>, Status> {
        Ok(Response::new(
            self.cancel(request.into_inner().try_into()?)
                .await
                .try_into()?,
        ))
    }
}

impl MatchingEngine {
    pub async fn submit(
        &self,
        request: SubmitRequest,
    ) -> Result<SubmitResponse, SubmitRequestError> {
        let quote_asset = self
            .assets_manager
            .get_by_id(request.quote_asset_id)
            .await?
            .ok_or(SubmitRequestError::AssetNotFound)?;
        let base_asset = self
            .assets_manager
            .get_by_id(request.base_asset_id)
            .await?
            .ok_or(SubmitRequestError::AssetNotFound)?;

        let mut taker_quote_asset_valut = self
            .valuts_manager
            .get_or_create(request.user_id, request.quote_asset_id)
            .await?;

        if taker_quote_asset_valut.balance < request.quote_asset_volume {
            return Err(SubmitRequestError::InsufficientBalance);
        }

        let matching_orders = self.orders_manager.get_orders_with_smaller_price(
            // base switches with quote to give opposite orderbook
            request.base_asset_id,
            request.quote_asset_id,
            // inverse of price
            request.price.clone().inv(),
        );

        let matching = self
            .matching_loop(
                request.user_id,
                request.price,
                request.quote_asset_volume.to_owned(),
                quote_asset,
                base_asset,
                matching_orders,
            )
            .await?;

        // saving to db

        taker_quote_asset_valut.balance -= request.quote_asset_volume;
        self.valuts_manager.update(taker_quote_asset_valut).await?;

        // apply order
        if let Some(order) = matching.order {
            self.orders_manager.insert(order).await?;
        }

        // apply trades
        for trade in matching.trades.into_iter() {
            let mut maker_order = self
                .orders_manager
                .get_by_id(trade.order_id)
                .await?
                .ok_or(SubmitRequestError::OrderNotFound)?;

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

            if maker_order.quote_asset_volume_left <= Fraction::zero() {
                maker_order.is_active = false;
            }

            // save changes
            self.orders_manager.update(maker_order.into()).await?;
            self.valuts_manager.update(taker_base_asset_valut).await?;
            self.valuts_manager.update(maker_base_asset_valut).await?;
            self.trades_manager.insert(trade).await?;
        }

        Ok(SubmitResponse {})
    }

    pub async fn cancel(
        &self,
        request: CancelRequest,
    ) -> Result<CancelResponse, CancelRequestError> {
        let mut order = self
            .orders_manager
            .get_by_id(request.order_id)
            .await?
            .ok_or(CancelRequestError::OrderNotFound)?;

        if !order.is_active {
            return Err(CancelRequestError::OrderNotActive);
        }

        let mut valut = self
            .valuts_manager
            .get_or_create(order.user_id, order.quote_asset_id)
            .await?;

        valut.balance += order.quote_asset_volume_left.to_owned();
        order.is_active = false;

        self.valuts_manager.update(valut).await?;
        self.orders_manager.update_status(order.into()).await?;

        Ok(CancelResponse {})
    }

    /*  matching engine axioms:
     *  maker strategy: buy required base asset volume for minimal amount of quote asset
     *  taker strategy: buy as much base asset volume as passible with given quote asset volume
     *  price is defined as quote_asset_volume / base_asset_volume
     */
    async fn matching_loop(
        &self,
        user_id: Uuid,
        price: Fraction,
        quote_asset_volume: Fraction,
        quote_asset: Asset,
        base_asset: Asset,
        mut matching_orders: Pin<Box<dyn Stream<Item = sqlx::Result<OrderGet>> + Send + '_>>,
    ) -> Result<MatchingLoopResponse, MatchingLoopError> {
        let mut response = MatchingLoopResponse::new();

        if quote_asset_volume <= Fraction::zero() {
            return Err(MatchingLoopError::VolumeIsZero);
        }

        let mut quote_asset_volume_left = quote_asset_volume.to_owned();
        let price_inv = price.clone().inv();

        while let Some(matching_order) = matching_orders.next().await {
            let maker_order = matching_order?;
            let maker_order_base_asset_volume_left =
                maker_order.quote_asset_volume_left.to_owned() * maker_order.price.to_owned().inv();

            if !(maker_order.price <= price_inv
                && maker_order.quote_asset_volume_left > Fraction::zero())
            {
                return Err(MatchingLoopError::InvalidData);
            }

            let (taker_base_asset_volume_given, taker_quote_asset_volume_taken) =
                if quote_asset_volume_left >= maker_order_base_asset_volume_left {
                    // eat whole maker_order
                    (
                        maker_order.quote_asset_volume_left,
                        maker_order_base_asset_volume_left,
                    )
                } else {
                    // eat maker_order partially
                    (
                        (quote_asset_volume_left.to_owned()
                            * maker_order.quote_asset_volume_left
                            * maker_order_base_asset_volume_left.inv()),
                        quote_asset_volume_left.to_owned(),
                    )
                };

            quote_asset_volume_left -= taker_quote_asset_volume_taken.to_owned();

            let (maker_base_asset_volume_given, maker_quote_asset_volume_taken) = (
                taker_quote_asset_volume_taken.to_owned(),
                taker_base_asset_volume_given.to_owned(),
            );

            response.trades.push(Trade {
                taker_id: user_id,
                order_id: maker_order.id,
                taker_quote_volume: taker_quote_asset_volume_taken,
                taker_base_volume: (taker_base_asset_volume_given.to_owned()
                    - taker_base_asset_volume_given * base_asset.taker_fee.to_owned())
                .floor_with_accuracy(self.accuracy.to_owned()),
                maker_quote_volume: maker_quote_asset_volume_taken,
                maker_base_volume: (maker_base_asset_volume_given.to_owned()
                    - maker_base_asset_volume_given * maker_order.maker_fee)
                    .floor_with_accuracy(self.accuracy.to_owned()),
            });
        }

        response.order = if quote_asset_volume_left > Fraction::zero() {
            Some(OrderInsert {
                user_id,
                quote_asset_id: quote_asset.id,
                base_asset_id: base_asset.id,
                price,
                quote_asset_volume,
                quote_asset_volume_left,
                maker_fee: base_asset.maker_fee,
            })
        } else {
            None
        };

        Ok(response)
    }
}
