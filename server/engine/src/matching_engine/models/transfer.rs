use std::str::FromStr;

use fraction::Fraction;
use sqlx::types::Uuid;
use thiserror::Error;
use tonic::Status;

use crate::base;

pub struct TransferRequest {
    pub maker_id: Uuid,
    pub taker_id: Uuid,
    pub asset_id: Uuid,
    pub amount: Fraction,
}

impl TryFrom<base::TransferRequest> for TransferRequest {
    type Error = Status;
    fn try_from(value: base::TransferRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            maker_id: Uuid::from_str(&value.maker_id)
                .map_err(|e| Status::invalid_argument(e.to_string()))?,
            taker_id: Uuid::from_str(&value.taker_id)
                .map_err(|e| Status::invalid_argument(e.to_string()))?,
            asset_id: Uuid::from_str(&value.asset_id)
                .map_err(|e| Status::invalid_argument(e.to_string()))?,
            amount: serde_json::from_str(&value.amount)
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

#[derive(Error, Debug)]
pub enum TransferError {
    #[error("asset not found in db")]
    AssetNotFound,

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
}

pub struct RevertTransferRequest {
    pub id: Uuid,
}

impl TryFrom<base::RevertTransferRequest> for RevertTransferRequest {
    type Error = Status;
    fn try_from(value: base::RevertTransferRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Uuid::from_str(&value.id).map_err(|e| Status::invalid_argument(e.to_string()))?,
        })
    }
}

pub struct RevertTransferResponse {}

impl TryFrom<Result<RevertTransferResponse, RevertTransferError>> for base::RevertTransferResponse {
    type Error = Status;
    fn try_from(
        value: Result<RevertTransferResponse, RevertTransferError>,
    ) -> Result<Self, Self::Error> {
        let _v = value.map_err(|e| Status::aborted(e.to_string()))?;
        Ok(Self {})
    }
}

#[derive(Error, Debug)]
pub enum RevertTransferError {
    #[error("asset not found in db")]
    AssetNotFound,

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
    TransferError(#[from] TransferError),
}
