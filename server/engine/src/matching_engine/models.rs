use std::str::FromStr;

use fraction::Fraction;
use sqlx::types::Uuid;
use tonic::Status;

use crate::{
    base,
    database::projections::{order::OrderInsert, trade::Trade},
};

pub struct SubmitRequest {
    pub user_id: Uuid,
    pub quote_asset_id: Uuid,
    pub base_asset_id: Uuid,
    pub price: Fraction,
    pub quote_asset_volume: Fraction,
    pub presentation: bool,
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
            presentation: serde_json::from_str(&value.presentation)
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

pub struct TransferRequest {
    pub maker_id: Uuid,
    pub taker_id: Uuid,
    pub asset: Uuid,
    pub volume: Fraction,
}

impl TryFrom<base::TransferRequest> for TransferRequest {
    type Error = Status;
    fn try_from(value: base::TransferRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            maker_id: Uuid::from_str(&value.maker_id)
                .map_err(|e| Status::invalid_argument(e.to_string()))?,
            taker_id: Uuid::from_str(&value.taker_id)
                .map_err(|e| Status::invalid_argument(e.to_string()))?,
            asset: Uuid::from_str(&value.asset)
                .map_err(|e| Status::invalid_argument(e.to_string()))?,
            volume: serde_json::from_str(&value.volume)
                .map_err(|e| Status::invalid_argument(e.to_string()))?,
        })
    }
}

pub struct TransferResponse {}

impl TryFrom<Result<TransferResponse, TransferError>> for base::TransferResponse {
    type Error = Status;
    fn try_from(value: Result<TransferResponse, TransferError>) -> Result<Self, Self::Error> {
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
    InvalidMatchingOrderData,

    #[error("add fractions failed")]
    CheckedAddFailed,

    #[error("sub fractions failed")]
    CheckedSubFailed,

    #[error("mul fractions failed")]
    CheckedMulFailed,

    #[error("div fractions failed")]
    CheckedDivFailed,

    #[error("floor fractions failed")]
    CheckedFloorFailed,

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

    #[error("add fractions failed")]
    CheckedAddFailed,

    #[error("sub fractions failed")]
    CheckedSubFailed,

    #[error("mul fractions failed")]
    CheckedMulFailed,

    #[error("div fractions failed")]
    CheckedDivFailed,

    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    #[error(transparent)]
    MatchingLoopError(#[from] MatchingLoopError),
}

#[derive(Error, Debug)]
pub enum TransferError {
    #[error("asset not found in db")]
    AssetNotFound,

    #[error("order not found in db")]
    OrderNotFound,

    #[error("not enough balance in valut")]
    InsufficientBalance,

    #[error("add fractions failed")]
    CheckedAddFailed,

    #[error("sub fractions failed")]
    CheckedSubFailed,

    #[error("mul fractions failed")]
    CheckedMulFailed,

    #[error("div fractions failed")]
    CheckedDivFailed,

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
