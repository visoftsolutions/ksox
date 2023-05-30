use std::pin::Pin;

use base::engine_server::Engine;
use fraction::{
    num_traits::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, One, Zero},
    Fraction,
};
use sqlx::{PgPool, Postgres, Transaction};
use tokio_stream::{Stream, StreamExt};
use tonic::{Request, Response, Status};
use uuid::Uuid;

use self::models::{
    CancelRequest, CancelRequestError, CancelResponse, MatchingLoopError, MatchingLoopResponse,
    SubmitRequest, SubmitRequestError, SubmitResponse, TransferError, TransferRequest,
    TransferResponse,
};
use crate::{
    base,
    database::{
        managers::{AssetsManager, OrdersManager, TradesManager, ValutsManager},
        projections::{
            asset::Asset,
            order::{OrderGet, OrderInsert},
            trade::Trade,
        },
    },
};
pub mod models;

#[cfg(test)]
pub mod tests;

pub struct MatchingEngine {
    accuracy: Fraction,
    database: PgPool,
}

impl MatchingEngine {
    pub fn new(database: PgPool, accuracy: Fraction) -> Self {
        Self { accuracy, database }
    }
}

#[tonic::async_trait]
impl Engine for MatchingEngine {
    async fn submit(
        &self,
        request: Request<base::SubmitRequest>,
    ) -> Result<Response<base::SubmitResponse>, Status> {
        let mut t = self
            .database
            .begin()
            .await
            .map_err(|e| Status::aborted(e.to_string()))?;
        Ok(Response::new(
            match self.submit(request.into_inner().try_into()?, &mut t).await {
                Ok(r) => {
                    t.commit()
                        .await
                        .map_err(|e| Status::aborted(e.to_string()))?;
                    Ok(r)
                }
                Err(e) => {
                    t.rollback()
                        .await
                        .map_err(|e| Status::aborted(e.to_string()))?;
                    Err(e)
                }
            }
            .try_into()?,
        ))
    }

    async fn transfer(
        &self,
        request: Request<base::TransferRequest>,
    ) -> Result<Response<base::TransferResponse>, Status> {
        let mut t = self
            .database
            .begin()
            .await
            .map_err(|e| Status::aborted(e.to_string()))?;
        Ok(Response::new(
            match self
                .transfer(request.into_inner().try_into()?, &mut t)
                .await
            {
                Ok(r) => {
                    t.commit()
                        .await
                        .map_err(|e| Status::aborted(e.to_string()))?;
                    Ok(r)
                }
                Err(e) => {
                    t.rollback()
                        .await
                        .map_err(|e| Status::aborted(e.to_string()))?;
                    Err(e)
                }
            }
            .try_into()?,
        ))
    }

    async fn cancel(
        &self,
        request: Request<base::CancelRequest>,
    ) -> Result<Response<base::CancelResponse>, Status> {
        let mut t = self
            .database
            .begin()
            .await
            .map_err(|e| Status::aborted(e.to_string()))?;
        Ok(Response::new(
            match self.cancel(request.into_inner().try_into()?, &mut t).await {
                Ok(r) => {
                    t.commit()
                        .await
                        .map_err(|e| Status::aborted(e.to_string()))?;
                    Ok(r)
                }
                Err(e) => {
                    t.rollback()
                        .await
                        .map_err(|e| Status::aborted(e.to_string()))?;
                    Err(e)
                }
            }
            .try_into()?,
        ))
    }
}

impl MatchingEngine {
    pub async fn submit<'t, 'p>(
        &self,
        request: SubmitRequest,
        transaction: &'t mut Transaction<'p, Postgres>,
    ) -> Result<SubmitResponse, SubmitRequestError> {
        let quote_asset = AssetsManager::get_by_id(transaction, request.quote_asset_id)
            .await?
            .ok_or(SubmitRequestError::AssetNotFound)?;
        let base_asset = AssetsManager::get_by_id(transaction, request.base_asset_id)
            .await?
            .ok_or(SubmitRequestError::AssetNotFound)?;

        let mut taker_quote_asset_valut =
            ValutsManager::get_or_create(transaction, request.user_id, request.quote_asset_id)
                .await?;

        if taker_quote_asset_valut.balance < request.quote_asset_volume {
            return Err(SubmitRequestError::InsufficientBalance);
        }

        let matching_orders = OrdersManager::get_orders_with_not_smaller_price(
            transaction,
            // base switches with quote to give opposite orderbook
            request.base_asset_id,
            request.quote_asset_id,
            // inverse of price
            Fraction::one()
                .checked_div(&request.price)
                .ok_or(SubmitRequestError::CheckedDivFailed)?,
        );

        let matching = Self::matching_loop(
            request.user_id,
            request.price,
            request.quote_asset_volume.to_owned(),
            quote_asset,
            base_asset,
            matching_orders,
            self.accuracy.to_owned(),
            request.presentation,
        )
        .await?;

        // saving to db

        taker_quote_asset_valut.balance = taker_quote_asset_valut
            .balance
            .checked_sub(&request.quote_asset_volume)
            .ok_or(SubmitRequestError::CheckedSubFailed)?;
        ValutsManager::update(transaction, taker_quote_asset_valut).await?;

        // apply order
        if let Some(order) = matching.order {
            OrdersManager::insert(transaction, order).await?;
        }

        // apply trades
        for trade in matching.trades.into_iter() {
            let mut maker_order = OrdersManager::get_by_id(transaction, trade.order_id)
                .await?
                .ok_or(SubmitRequestError::OrderNotFound)?;

            let mut taker_base_asset_valut = ValutsManager::get_or_create(
                transaction,
                trade.taker_id,
                maker_order.quote_asset_id,
            )
            .await?;
            let mut maker_base_asset_valut = ValutsManager::get_or_create(
                transaction,
                maker_order.maker_id,
                maker_order.base_asset_id,
            )
            .await?;

            // apply changes
            taker_base_asset_valut.balance = taker_base_asset_valut
                .balance
                .checked_add(&trade.taker_base_volume)
                .ok_or(SubmitRequestError::CheckedAddFailed)?;
            maker_base_asset_valut.balance = maker_base_asset_valut
                .balance
                .checked_add(&trade.maker_base_volume)
                .ok_or(SubmitRequestError::CheckedAddFailed)?;
            maker_order.quote_asset_volume_left = maker_order
                .quote_asset_volume_left
                .checked_sub(&trade.maker_quote_volume)
                .ok_or(SubmitRequestError::CheckedSubFailed)?;

            if maker_order.quote_asset_volume_left <= Fraction::zero() {
                maker_order.is_active = false;
            }

            // save changes
            OrdersManager::update(transaction, maker_order.into()).await?;
            ValutsManager::update(transaction, taker_base_asset_valut).await?;
            ValutsManager::update(transaction, maker_base_asset_valut).await?;
            TradesManager::insert(transaction, trade).await?;
        }

        Ok(SubmitResponse {})
    }

    pub async fn transfer<'t, 'p>(
        &self,
        request: TransferRequest,
        transaction: &'t mut Transaction<'p, Postgres>,
    ) -> Result<TransferResponse, TransferError> {
        let asset = AssetsManager::get_by_id(transaction, request.asset)
            .await?
            .ok_or(TransferError::AssetNotFound)?;

        let mut maker_asset_valut =
            ValutsManager::get_or_create(transaction, request.maker, asset.id).await?;

        let mut taker_asset_valut =
            ValutsManager::get_or_create(transaction, request.taker, asset.id).await?;

        if maker_asset_valut.balance < request.volume {
            return Err(TransferError::InsufficientBalance);
        }

        // transfer
        maker_asset_valut.balance = maker_asset_valut
            .balance
            .checked_sub(&request.volume)
            .ok_or(TransferError::CheckedSubFailed)?;
        taker_asset_valut.balance = taker_asset_valut
            .balance
            .checked_add(&request.volume)
            .ok_or(TransferError::CheckedAddFailed)?;

        ValutsManager::update(transaction, maker_asset_valut).await?;
        ValutsManager::update(transaction, taker_asset_valut).await?;

        Ok(TransferResponse {})
    }

    pub async fn cancel<'t, 'p>(
        &self,
        request: CancelRequest,
        transaction: &'t mut Transaction<'p, Postgres>,
    ) -> Result<CancelResponse, CancelRequestError> {
        let mut order = OrdersManager::get_by_id(transaction, request.order_id)
            .await?
            .ok_or(CancelRequestError::OrderNotFound)?;

        if !order.is_active {
            return Err(CancelRequestError::OrderNotActive);
        }

        let mut valut =
            ValutsManager::get_or_create(transaction, order.maker_id, order.quote_asset_id).await?;

        valut.balance += order.quote_asset_volume_left.to_owned();
        order.is_active = false;

        ValutsManager::update(transaction, valut).await?;
        OrdersManager::update_status(transaction, order.into()).await?;

        Ok(CancelResponse {})
    }

    /*  matching engine axioms:
     *  maker strategy: buy required base asset volume for minimal amount of quote asset
     *  taker strategy: buy as much base asset volume as passible with given quote asset volume
     *  price is defined as quote_asset_volume / base_asset_volume
     */
    async fn matching_loop(
        user_id: Uuid,
        price: Fraction,
        quote_asset_volume: Fraction,
        quote_asset: Asset,
        base_asset: Asset,
        mut matching_orders: Pin<Box<dyn Stream<Item = sqlx::Result<OrderGet>> + Send + '_>>,
        accuracy: Fraction,
        presentation: bool,
    ) -> Result<MatchingLoopResponse, MatchingLoopError> {
        let mut response = MatchingLoopResponse::new();

        if quote_asset_volume <= Fraction::zero() {
            return Err(MatchingLoopError::VolumeIsZero);
        }

        let mut quote_asset_volume_left = quote_asset_volume.to_owned();

        let price_inv = Fraction::one()
            .checked_div(&price)
            .ok_or(MatchingLoopError::CheckedDivFailed)?;

        while let Some(matching_order) = matching_orders.next().await && quote_asset_volume_left > Fraction::zero() {
            let maker_order = matching_order?;

            if !(maker_order.price >= price_inv
                && maker_order.quote_asset_volume_left > Fraction::zero())
            {
                return Err(MatchingLoopError::InvalidMatchingOrderData);
            }

            let maker_order_base_asset_volume_left = maker_order
                .quote_asset_volume_left
                .checked_div(&maker_order.price)
                .ok_or(MatchingLoopError::CheckedDivFailed)?;

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
                        quote_asset_volume_left
                            .checked_mul(&maker_order.quote_asset_volume_left)
                            .ok_or(MatchingLoopError::CheckedMulFailed)?
                            .checked_div(&maker_order_base_asset_volume_left)
                            .ok_or(MatchingLoopError::CheckedDivFailed)?,
                        quote_asset_volume_left.to_owned(),
                    )
                };

            quote_asset_volume_left = quote_asset_volume_left
                .checked_sub(&taker_quote_asset_volume_taken)
                .ok_or(MatchingLoopError::CheckedSubFailed)?;

            let (maker_base_asset_volume_given, maker_quote_asset_volume_taken) = (
                taker_quote_asset_volume_taken.to_owned(),
                taker_base_asset_volume_given.to_owned(),
            );

            response.trades.push(Trade {
                quote_asset_id: quote_asset.id,
                base_asset_id: base_asset.id,
                taker_id: user_id,
                taker_presentation: presentation,
                order_id: maker_order.id,
                taker_price: Fraction::one().checked_div(&maker_order.price).ok_or(MatchingLoopError::CheckedDivFailed)?,
                taker_quote_volume: taker_quote_asset_volume_taken,
                taker_base_volume: taker_base_asset_volume_given
                    .checked_sub(
                        &taker_base_asset_volume_given
                            .checked_mul(&base_asset.taker_fee)
                            .ok_or(MatchingLoopError::CheckedMulFailed)?,
                    )
                    .ok_or(MatchingLoopError::CheckedSubFailed)?
                    .checked_floor_with_accuracy(&accuracy)
                    .ok_or(MatchingLoopError::CheckedFloorFailed)?,
                maker_quote_volume: maker_quote_asset_volume_taken,
                maker_base_volume: maker_base_asset_volume_given
                    .checked_sub(
                        &maker_base_asset_volume_given
                            .checked_mul(&maker_order.maker_fee)
                            .ok_or(MatchingLoopError::CheckedMulFailed)?,
                    )
                    .ok_or(MatchingLoopError::CheckedSubFailed)?
                    .checked_floor_with_accuracy(&accuracy)
                    .ok_or(MatchingLoopError::CheckedFloorFailed)?,
            });
        }

        response.order = if quote_asset_volume_left > Fraction::zero() {
            Some(OrderInsert {
                maker_id: user_id,
                maker_presentation: presentation,
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
