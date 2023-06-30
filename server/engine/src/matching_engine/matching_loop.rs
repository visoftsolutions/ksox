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
    trade::Trade,
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
            taker_price: Fraction::one()
                .checked_div(&maker_order.price)
                .ok_or(MatchingLoopError::CheckedDivFailed)?,
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
