use std::str::FromStr;

use sqlx::types::Uuid;
use thiserror::Error;
use tonic::Status;

use crate::base;

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

#[derive(Error, Debug)]
pub enum CancelRequestError {
    #[error("order not found in db")]
    OrderNotFound,

    #[error("order not active")]
    OrderNotActive,

    #[error("add fractions failed")]
    CheckedAddFailed,

    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}
