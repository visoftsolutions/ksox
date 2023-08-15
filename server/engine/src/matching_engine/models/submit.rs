use std::str::FromStr;

use fraction::Fraction;
use sqlx::types::Uuid;
use thiserror::Error;
use tonic::Status;

use super::{transfer::TransferError, MatchingLoopError};
use crate::base;

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

    #[error("transfer failed")]
    TransferError(#[from] TransferError),

    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    #[error(transparent)]
    MatchingLoopError(#[from] MatchingLoopError),
}
