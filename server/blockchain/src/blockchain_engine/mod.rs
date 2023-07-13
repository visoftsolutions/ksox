pub mod deposits;
pub mod models;
pub mod valuts;
pub mod withdraws;

use ethers::{
    prelude::{k256::ecdsa::SigningKey, SignerMiddleware},
    providers::{Provider, Ws},
    signers::Wallet,
};
use tonic::{Request, Response, Status};

use crate::{
    base::{self, blockchain_server::Blockchain},
    blockchain_engine::models::withdraw::WithdrawRequest,
    contracts::treasury::Treasury,
};

use self::{
    deposits::DepositsBlockchainManagerController, valuts::ValutsBlockchainManagerController,
    withdraws::WithdrawsBlockchainManagerController,
};

#[derive(Debug)]
pub struct BlockchainEngine {
    pub contract: Treasury<SignerMiddleware<Provider<Ws>, Wallet<SigningKey>>>,
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
        let request: WithdrawRequest = request.into_inner().try_into()?;
        todo!()
    }
}
