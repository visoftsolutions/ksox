use std::pin::Pin;

use fraction::{
    num_traits::{CheckedDiv, CheckedMul, CheckedSub, One, Zero},
    Fraction,
};
use tokio_stream::{Stream, StreamExt};
use uuid::Uuid;

use super::models::{MatchingLoopError, MatchingLoopResponse};
use crate::database::projections::{
    asset::Asset,
    order::{OrderGet, OrderInsert},
    trade::TradeInsert,
};

/*  matching engine axioms:
 *  maker strategy: buy required base asset volume for minimal amount of quote asset
 *  taker strategy: buy as much base asset volume as passible with given quote asset volume
 *  price is defined as quote_asset_volume / base_asset_volume
 */
pub async fn matching_loop(
    user_id: Uuid,
    price: Fraction,
    quote_asset_volume: Fraction,
    quote_asset: Asset,
    base_asset: Asset,
    mut matching_orders: Pin<Box<dyn Stream<Item = sqlx::Result<OrderGet>> + Send + '_>>,
    presentation: bool,
) -> Result<MatchingLoopResponse, MatchingLoopError> {
    let mut response = MatchingLoopResponse::new();

    if quote_asset_volume <= Fraction::zero() {
        return Err(MatchingLoopError::VolumeIsZero);
    }

    let quote_asset_decimals_inv = Fraction::one()
        .checked_div(&quote_asset.decimals)
        .ok_or(MatchingLoopError::CheckedDivFailed)?;

    let base_asset_decimals_inv = Fraction::one()
        .checked_div(&base_asset.decimals)
        .ok_or(MatchingLoopError::CheckedDivFailed)?;

    let price_inv = Fraction::one()
        .checked_div(&price)
        .ok_or(MatchingLoopError::CheckedDivFailed)?;

    let mut quote_asset_volume_left = quote_asset_volume
        .to_owned()
        .checked_floor_with_accuracy(&quote_asset_decimals_inv)
        .ok_or(MatchingLoopError::CheckedFloorFailed)?;

    while let Some(matching_order) = matching_orders.next().await {
        if quote_asset_volume_left <= Fraction::zero() {
            break;
        }

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
                    maker_order
                        .quote_asset_volume_left
                        .checked_floor_with_accuracy(&base_asset_decimals_inv)
                        .ok_or(MatchingLoopError::CheckedFloorFailed)?,
                    maker_order_base_asset_volume_left
                        .checked_floor_with_accuracy(&quote_asset_decimals_inv)
                        .ok_or(MatchingLoopError::CheckedFloorFailed)?,
                )
            } else {
                // eat maker_order partially
                (
                    quote_asset_volume_left
                        .checked_mul(&maker_order.quote_asset_volume_left)
                        .ok_or(MatchingLoopError::CheckedMulFailed)?
                        .checked_div(&maker_order_base_asset_volume_left)
                        .ok_or(MatchingLoopError::CheckedDivFailed)?
                        .checked_floor_with_accuracy(&base_asset_decimals_inv)
                        .ok_or(MatchingLoopError::CheckedFloorFailed)?,
                    quote_asset_volume_left.to_owned(),
                )
            };

        quote_asset_volume_left = quote_asset_volume_left
            .checked_sub(&taker_quote_asset_volume_taken)
            .ok_or(MatchingLoopError::CheckedSubFailed)?;

        let maker_quote_asset_volume_taken = taker_base_asset_volume_given;

        response.trades.push(TradeInsert {
            quote_asset_id: quote_asset.id,
            base_asset_id: base_asset.id,
            taker_id: user_id,
            maker_id: maker_order.maker_id,
            taker_presentation: presentation,
            order_id: maker_order.id,
            taker_price: Fraction::one()
                .checked_div(&maker_order.price)
                .ok_or(MatchingLoopError::CheckedDivFailed)?,
            taker_quote_volume: taker_quote_asset_volume_taken,
            maker_quote_volume: maker_quote_asset_volume_taken,
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
