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
        deposits::models::DepositQueueError, models::BlockchainEngineError,
        withdraws::models::WithdrawQueueValue,
    },
    database::projections::withdraw::WithdrawInsert,
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
    DepositQueueError(#[from] DepositQueueError),

    #[error(transparent)]
    BlockchainEngineError(#[from] BlockchainEngineError),

    #[error(transparent)]
    SendError(#[from] tokio::sync::mpsc::error::SendError<(WithdrawInsert, WithdrawQueueValue)>),

    #[error(transparent)]
    TimeError(#[from] TimeError),

    #[error(transparent)]
    Uuid(#[from] uuid::Error),
}
