use std::pin::Pin;

use database::{
    projections::spot::{order::Order, trade::Trade},
    sqlx::{error::Error, types::Uuid},
    types::{fraction::FractionError, Fraction, Volume},
};
use futures::Stream;
use thiserror::Error;

pub struct MatchingEngineRequest {
    pub user_id: Uuid,
    pub quote_asset_id: Uuid,
    pub base_asset_id: Uuid,
    pub quote_asset_volume: Volume,
    pub base_asset_volume: Volume,
}

#[derive(Debug)]
pub struct MatchingEngineResponse {
    pub order: Option<Order>,
    pub trades: Vec<Trade>,
}

impl MatchingEngineResponse {
    pub fn new() -> MatchingEngineResponse {
        MatchingEngineResponse {
            order: None,
            trades: Vec::new(),
        }
    }
}

impl std::fmt::Display for MatchingEngineResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MatchingEngineResponse:
        order: {:?}
        trades: {:?}
        ",
            self.order, self.trades
        )
    }
}

pub struct MatchingEngineData<'a> {
    pub user_id: Uuid,
    pub quote_asset_id: Uuid,
    pub base_asset_id: Uuid,
    pub quote_asset_volume: Volume,
    pub base_asset_volume: Volume,
    pub maker_orders: Pin<Box<dyn Stream<Item = Result<Order, Error>> + Send + 'a>>,
    pub taker_fee: Fraction,
    pub maker_fee: Fraction,
}

impl<'a> MatchingEngineData<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        user_id: Uuid,
        quote_asset_id: Uuid,
        base_asset_id: Uuid,
        quote_asset_volume: Volume,
        base_asset_volume: Volume,
        maker_orders: Pin<Box<dyn Stream<Item = Result<Order, Error>> + Send + 'a>>,
        taker_fee: Fraction,
        maker_fee: Fraction,
    ) -> Self {
        Self {
            user_id,
            quote_asset_id,
            base_asset_id,
            quote_asset_volume,
            base_asset_volume,
            maker_orders,
            taker_fee,
            maker_fee,
        }
    }
}

#[derive(Error, Debug)]
pub enum MatchingEngineError {
    #[error("division by zero")]
    DivisionByZero,

    #[error("volume value can not be zero")]
    VolumeIsZero,

    #[error("not enough balance")]
    InsufficientBalance,

    #[error("order not active")]
    NotActive,

    // source and Display delegate to sqlx::error::Error
    #[error(transparent)]
    Sqlx(#[from] Error),

    // source and Display delegate to Fraction
    #[error(transparent)]
    Fraction(#[from] FractionError),
}
