use crate::matching_engine::transfer::transfer;
use fraction::{
    num_traits::{CheckedDiv, CheckedSub, One, Zero},
    Fraction,
};
use sqlx::{Postgres, Transaction};
use uuid::Uuid;
use value::Value;

use super::{
    matching_loop::matching_loop,
    models::{
        submit::{SubmitRequest, SubmitRequestError, SubmitResponse},
        transfer::TransferRequest,
    },
};
use crate::database::{
    managers::{AssetsManager, OrdersManager, TradesManager, ValutsManager},
    projections::trade::Trade,
};

pub async fn submit<'t>(
    request: SubmitRequest,
    fee_harvester_user_id: &Uuid,
    transaction: &'t mut Transaction<'_, Postgres>,
) -> Result<SubmitResponse, SubmitRequestError> {
    let taker_quote_asset = AssetsManager::get_by_id(transaction, &request.quote_asset_id)
        .await?
        .ok_or(SubmitRequestError::AssetNotFound)?;
    let taker_base_asset = AssetsManager::get_by_id(transaction, &request.base_asset_id)
        .await?
        .ok_or(SubmitRequestError::AssetNotFound)?;

    let taker_quote_asset_valut =
        ValutsManager::get_or_create(transaction, &request.user_id, &request.quote_asset_id)
            .await?;

    let request_quote_asset_volume = Value::Finite(request.quote_asset_volume.to_owned());

    if taker_quote_asset_valut.balance < request_quote_asset_volume {
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
        taker_quote_asset.to_owned(),
        taker_base_asset.to_owned(),
        matching_orders,
        request.presentation,
    )
    .await?;

    // apply order
    if let Some(order) = matching.order {
        let mut valut =
            ValutsManager::get_or_create(transaction, &order.maker_id, &order.quote_asset_id)
                .await?;
        valut.balance = valut
            .balance
            .checked_sub(&Value::Finite(order.quote_asset_volume_left.to_owned()))
            .ok_or(SubmitRequestError::CheckedAddFailed)?;
        ValutsManager::update(transaction, valut).await?;
        OrdersManager::insert(transaction, order).await?;
    }

    // apply trades
    for trade_insert in matching.trades.into_iter() {
        let mut maker_order = OrdersManager::get_by_id(transaction, trade_insert.order_id)
            .await?
            .ok_or(SubmitRequestError::OrderNotFound)?;

        let taker_quote_asset_valut = ValutsManager::get_or_create(
            transaction,
            &trade_insert.taker_id,
            &request.quote_asset_id,
        )
        .await?;

        let taker_base_asset_valut = ValutsManager::get_or_create(
            transaction,
            &trade_insert.taker_id,
            &request.base_asset_id,
        )
        .await?;

        let maker_quote_asset_valut = ValutsManager::get_or_create(
            transaction,
            &maker_order.maker_id,
            &maker_order.quote_asset_id,
        )
        .await?;

        let maker_base_asset_valut = ValutsManager::get_or_create(
            transaction,
            &maker_order.maker_id,
            &maker_order.base_asset_id,
        )
        .await?;

        // apply changes
        let taker_quote_volume_transfer = transfer(
            &TransferRequest {
                from_valut_id: taker_quote_asset_valut.id,
                to_valut_id: maker_base_asset_valut.id,
                asset_id: taker_quote_asset.id,
                amount: trade_insert.taker_quote_volume.to_owned(),
                fee_fraction: taker_quote_asset.maker_fee.to_owned(),
            },
            fee_harvester_user_id,
            transaction,
        )
        .await?;

        let maker_quote_volume_transfer = transfer(
            &TransferRequest {
                from_valut_id: maker_quote_asset_valut.id,
                to_valut_id: taker_base_asset_valut.id,
                asset_id: maker_order.quote_asset_id,
                amount: trade_insert.maker_quote_volume.to_owned(),
                fee_fraction: taker_base_asset.taker_fee.to_owned(),
            },
            fee_harvester_user_id,
            transaction,
        )
        .await?;

        let base_asset_decimals_inv = Fraction::one()
            .checked_div(&taker_base_asset.decimals)
            .ok_or(SubmitRequestError::CheckedDivFailed)?;

        maker_order.quote_asset_volume_left = maker_order
            .quote_asset_volume_left
            .checked_sub(&trade_insert.maker_quote_volume)
            .ok_or(SubmitRequestError::CheckedSubFailed)?
            .checked_floor_with_accuracy(&base_asset_decimals_inv)
            .ok_or(SubmitRequestError::CheckedAddFailed)?;

        if maker_order.quote_asset_volume_left <= Fraction::zero() {
            maker_order.is_active = false;
        }

        // save changes
        OrdersManager::update(transaction, maker_order.into()).await?;
        TradesManager::insert(
            transaction,
            Trade {
                quote_asset_id: trade_insert.quote_asset_id,
                base_asset_id: trade_insert.base_asset_id,
                taker_id: trade_insert.taker_id,
                taker_presentation: trade_insert.taker_presentation,
                order_id: trade_insert.order_id,
                taker_price: trade_insert.taker_price,
                taker_quote_volume: trade_insert.taker_quote_volume,
                maker_quote_volume: trade_insert.maker_quote_volume,
                taker_quote_volume_transfer_id: taker_quote_volume_transfer.transfer_id,
                maker_quote_volume_transfer_id: maker_quote_volume_transfer.transfer_id,
            },
        )
        .await?;
    }

    Ok(SubmitResponse {})
}
