use std::str::FromStr;

use sqlx::types::Uuid;
use tonic::Status;

use crate::{
    base,
    database::projections::{OrderInsert, Trade},
    types::Fraction,
};

pub struct SubmitRequest {
    pub user_id: Uuid,
    pub quote_asset_id: Uuid,
    pub base_asset_id: Uuid,
    pub price: Fraction,
    pub quote_asset_volume: Fraction,
}

impl TryFrom<base::SubmitRequest> for SubmitRequest {
    type Error = Status;
    fn try_from(value: base::SubmitRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            user_id: Uuid::from_str(&value.user_id)
                .map_err(|e| Status::invalid_argument(e.to_string()))?,
            quote_asset_id: Uuid::from_str(&value.quote_asset_id)
                .map_err(|e| Status::invalid_argument(e.to_string()))?,
            base_asset_id: Uuid::from_str(&value.base_asset_id)
                .map_err(|e| Status::invalid_argument(e.to_string()))?,
            price: serde_json::from_str(&value.price)
                .map_err(|e| Status::invalid_argument(e.to_string()))?,
            quote_asset_volume: serde_json::from_str(&value.quote_asset_volume)
                .map_err(|e| Status::invalid_argument(e.to_string()))?,
        })
    }
}

pub struct SubmitResponse {}

impl TryFrom<Result<SubmitResponse, SubmitRequestError>> for base::SubmitResponse {
    type Error = Status;
    fn try_from(value: Result<SubmitResponse, SubmitRequestError>) -> Result<Self, Self::Error> {
        let _v = value.map_err(|e| Status::aborted(e.to_string()))?;
        Ok(Self {})
    }
}

pub struct CancelRequest {
    pub order_id: Uuid,
}

impl TryFrom<base::CancelRequest> for CancelRequest {
    type Error = Status;
    fn try_from(value: base::CancelRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            order_id: Uuid::from_str(&value.order_id)
                .map_err(|e| Status::invalid_argument(e.to_string()))?,
        })
    }
}

pub struct CancelResponse {}

impl TryFrom<Result<CancelResponse, CancelRequestError>> for base::CancelResponse {
    type Error = Status;
    fn try_from(value: Result<CancelResponse, CancelRequestError>) -> Result<Self, Self::Error> {
        let _v = value.map_err(|e| Status::aborted(e.to_string()))?;
        Ok(Self {})
    }
}

#[derive(Debug)]
pub struct MatchingLoopResponse {
    pub order: Option<OrderInsert>,
    pub trades: Vec<Trade>,
}

impl MatchingLoopResponse {
    pub fn new() -> Self {
        Self {
            order: None,
            trades: Vec::new(),
        }
    }
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum MatchingLoopError {
    #[error("volume can not be zero")]
    VolumeIsZero,

    #[error("matching orders invalid")]
    InvalidData,

    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}

#[derive(Error, Debug)]
pub enum SubmitRequestError {
    #[error("asset not found in db")]
    AssetNotFound,

    #[error("order not found in db")]
    OrderNotFound,

    #[error("not enough balance in valut")]
    InsufficientBalance,

    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    #[error(transparent)]
    MatchingLoopError(#[from] MatchingLoopError),
}

#[derive(Error, Debug)]
pub enum CancelRequestError {
    #[error("order not found in db")]
    OrderNotFound,

    #[error("order not active")]
    OrderNotActive,

    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}
