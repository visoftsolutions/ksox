use ethers::{
    prelude::{k256::ecdsa::SigningKey, ContractError, SignerMiddleware},
    providers::{Provider, ProviderError, Ws},
    signers::Wallet,
};
use thiserror::Error;
use tonic::Status;

use crate::{confirmation::ConfirmationQueueError, submission::SubmissionQueueError};

#[derive(Error, Debug)]
pub enum BlockchainManagerError {
    #[error(transparent)]
    ContractError(#[from] ContractError<SignerMiddleware<Provider<Ws>, Wallet<SigningKey>>>),

    #[error(transparent)]
    ProviderError(#[from] ProviderError),

    #[error(transparent)]
    Status(#[from] Status),

    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    #[error(transparent)]
    SubmissionQueueError(#[from] SubmissionQueueError),

    #[error(transparent)]
    ConfirmationQueueError(#[from] ConfirmationQueueError),
}
