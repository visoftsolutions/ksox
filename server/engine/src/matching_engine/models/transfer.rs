use std::str::FromStr;

use fraction::Fraction;
use sqlx::types::Uuid;
use thiserror::Error;
use tonic::Status;

use crate::base;

pub struct TransferRequest {
    pub from_valut_id: Uuid,
    pub to_valut_id: Uuid,
    pub asset_id: Uuid,
    pub amount: Fraction,
    pub fee_fraction: Fraction,
}

impl TryFrom<base::TransferRequest> for TransferRequest {
    type Error = Status;
    fn try_from(value: base::TransferRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            from_valut_id: Uuid::from_str(&value.from_valut_id)
                .map_err(|e| Status::invalid_argument(e.to_string()))?,
            to_valut_id: Uuid::from_str(&value.to_valut_id)
                .map_err(|e| Status::invalid_argument(e.to_string()))?,
            asset_id: Uuid::from_str(&value.asset_id)
                .map_err(|e| Status::invalid_argument(e.to_string()))?,
            amount: serde_json::from_str(&value.amount)
                .map_err(|e| Status::invalid_argument(e.to_string()))?,
            fee_fraction: serde_json::from_str(&value.fee_fraction)
                .map_err(|e| Status::invalid_argument(e.to_string()))?,
        })
    }
}

pub struct TransferResponse {
    pub transfer_id: Uuid,
}

impl TryFrom<Result<TransferResponse, TransferError>> for base::TransferResponse {
    type Error = Status;
    fn try_from(value: Result<TransferResponse, TransferError>) -> Result<Self, Self::Error> {
        let v = value.map_err(|e| Status::aborted(e.to_string()))?;
        Ok(Self {
            transfer_id: v.transfer_id.to_string(),
        })
    }
}

#[derive(Error, Debug)]
pub enum TransferError {
    #[error("asset not found in db")]
    AssetNotFound,

    #[error("not enough balance in valut")]
    InsufficientBalance,

    #[error("not enough balance in fee valut")]
    InsufficientBalanceFee,

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

    #[error("from to fee are not diffirent")]
    SameValut,

    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}

pub struct RevertTransferRequest {
    pub transfer_id: Uuid,
}

impl TryFrom<base::RevertTransferRequest> for RevertTransferRequest {
    type Error = Status;
    fn try_from(value: base::RevertTransferRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            transfer_id: Uuid::from_str(&value.transfer_id)
                .map_err(|e| Status::invalid_argument(e.to_string()))?,
        })
    }
}

pub struct RevertTransferResponse {
    pub amount_transfer_id: Uuid,
    pub fee_transfer_id: Uuid,
}

impl TryFrom<Result<RevertTransferResponse, RevertTransferError>> for base::RevertTransferResponse {
    type Error = Status;
    fn try_from(
        value: Result<RevertTransferResponse, RevertTransferError>,
    ) -> Result<Self, Self::Error> {
        let v = value.map_err(|e| Status::aborted(e.to_string()))?;
        Ok(Self {
            amount_transfer_id: v.amount_transfer_id.to_string(),
            fee_transfer_id: v.fee_transfer_id.to_string(),
        })
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
