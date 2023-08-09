use ethers::{
    prelude::ContractError,
    providers::{Provider, ProviderError, Ws},
    signers::WalletError,
    types::{transaction::eip712::Eip712Error, TimeError},
};
use thiserror::Error;
use tonic::Status;

use crate::{
    blockchain_engine::{
        models::BlockchainEngineError,
        withdraws::models::{WithdrawQueueKey, WithdrawQueueValue},
    },
    confirmation::ConfirmationQueueError,
};

#[derive(Error, Debug)]
pub enum BlockchainManagerError {
    #[error(transparent)]
    ContractError(#[from] ContractError<Provider<Ws>>),

    #[error(transparent)]
    ProviderError(#[from] ProviderError),

    #[error(transparent)]
    Status(#[from] Status),

    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    #[error(transparent)]
    Eip712Error(#[from] Eip712Error),

    #[error(transparent)]
    WalletError(#[from] WalletError),

    #[error(transparent)]
    ConfirmationQueueError(#[from] ConfirmationQueueError),

    #[error(transparent)]
    BlockchainEngineError(#[from] BlockchainEngineError),

    #[error(transparent)]
    SendError(#[from] tokio::sync::mpsc::error::SendError<(WithdrawQueueKey, WithdrawQueueValue)>),

    #[error(transparent)]
    TimeError(#[from] TimeError),

    #[error(transparent)]
    Uuid(#[from] uuid::Error),
}
