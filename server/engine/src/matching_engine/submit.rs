use fraction::{
    num_traits::{CheckedAdd, CheckedDiv, CheckedSub, One, Zero},
    Fraction,
};
use sqlx::{Postgres, Transaction};

use super::{
    matching_loop::matching_loop,
    models::submit::{SubmitRequest, SubmitRequestError, SubmitResponse},
};
use crate::database::managers::{AssetsManager, OrdersManager, TradesManager, ValutsManager};

pub async fn submit<'t, 'p>(
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
        ValutsManager::get_or_create(transaction, request.user_id, request.quote_asset_id).await?;

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

    let matching = matching_loop(
        request.user_id,
        request.price,
        request.quote_asset_volume.to_owned(),
        quote_asset,
        base_asset,
        matching_orders,
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

        let mut taker_base_asset_valut =
            ValutsManager::get_or_create(transaction, trade.taker_id, maker_order.quote_asset_id)
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
