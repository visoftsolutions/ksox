use thiserror::Error;

#[derive(Error, Debug)]
pub enum BlockchainEngineError {
    #[error(transparent)]
    SendError(#[from] tonic::Status),

    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}
