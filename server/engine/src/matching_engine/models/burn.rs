use std::str::FromStr;

use fraction::Fraction;
use sqlx::types::Uuid;
use thiserror::Error;
use tonic::Status;

use crate::base;

pub struct BurnRequest {
    pub user_id: Uuid,
    pub asset_id: Uuid,
    pub amount: Fraction,
}

impl TryFrom<base::BurnRequest> for BurnRequest {
    type Error = Status;
    fn try_from(value: base::BurnRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            user_id: Uuid::from_str(&value.user_id)
                .map_err(|e| Status::invalid_argument(e.to_string()))?,
            asset_id: Uuid::from_str(&value.asset_id)
                .map_err(|e| Status::invalid_argument(e.to_string()))?,
            amount: serde_json::from_str(&value.amount)
                .map_err(|e| Status::invalid_argument(e.to_string()))?,
        })
    }
}

impl TryInto<base::BurnRequest> for BurnRequest {
    type Error = Status;
    fn try_into(self) -> Result<base::BurnRequest, Self::Error> {
        Ok(base::BurnRequest {
            user_id: self.user_id.to_string(),
            asset_id: self.asset_id.to_string(),
            amount: serde_json::to_string(&self.amount)
                .map_err(|e| Status::invalid_argument(e.to_string()))?,
        })
    }
}

pub struct BurnResponse {}

impl TryFrom<Result<BurnResponse, BurnError>> for base::BurnResponse {
    type Error = Status;
    fn try_from(value: Result<BurnResponse, BurnError>) -> Result<Self, Self::Error> {
        let _v = value.map_err(|e| Status::aborted(e.to_string()))?;
        Ok(Self {})
    }
}

#[derive(Error, Debug)]
pub enum BurnError {
    #[error("invalid amount value supplied")]
    InvalidAmount,

    #[error("operation not available yet")]
    Timeout,

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
