use database::{sqlx, types::fraction::FractionError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MatchingEngineError {
    #[error("division by zero")]
    DivisionByZero,

    // source and Display delegate to sqlx::error::Error
    #[error(transparent)]
    Sqlx(#[from] sqlx::error::Error),

    // source and Display delegate to Fraction
    #[error(transparent)]
    Fraction(#[from] FractionError),
}
