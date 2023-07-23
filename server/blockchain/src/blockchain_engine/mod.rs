pub mod deposits;
pub mod valuts;
pub mod withdraws;

use ethers::{
    prelude::{k256::ecdsa::SigningKey, SignerMiddleware},
    providers::{Provider, Ws},
    signers::Wallet,
};
use sqlx::PgPool;
use thiserror::Error;
use tonic::{Request, Response, Status};

use crate::{
    base::{self, blockchain_server::Blockchain},
    contracts::treasury::Treasury,
};

use self::{
    deposits::DepositsBlockchainManagerController,
    valuts::ValutsBlockchainManagerController,
    withdraws::{models::{WithdrawEvent, WithdrawRequest}, WithdrawsBlockchainManagerController},
};

#[derive(Debug)]
pub struct BlockchainEngine {
    pub contract: Treasury<SignerMiddleware<Provider<Ws>, Wallet<SigningKey>>>,
    pub database: PgPool,
    pub deposits_controller: DepositsBlockchainManagerController,
    pub withdraws_controller: WithdrawsBlockchainManagerController,
    pub valuts_controller: ValutsBlockchainManagerController,
}

#[tonic::async_trait]
impl Blockchain for BlockchainEngine {
    async fn withdraw(
        &self,
        request: Request<base::WithdrawRequest>,
    ) -> Result<Response<base::WithdrawResponse>, Status> {
        self.withdraws_controller.withdraw(
            request.into_inner().try_into()?
        ).await.map_err(|e| Status::aborted(e.to_string()))?;
        Ok(Response::new(base::WithdrawResponse {}))
    }
}

#[derive(Error, Debug)]
pub enum BlockchainEngineError {
    // #[error(transparent)]
    // Tonic(#[from] tonic::Status),

    #[error(transparent)]
    SendError(#[from] tonic::Status),
}
