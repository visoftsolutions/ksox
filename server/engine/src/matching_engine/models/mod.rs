pub mod cancel;
pub mod submit;
pub mod transfer;

use thiserror::Error;

use crate::database::projections::{order::OrderInsert, trade::Trade};

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

impl Default for MatchingLoopResponse {
    fn default() -> Self {
        Self::new()
    }
}

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
