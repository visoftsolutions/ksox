pub mod deposits;
pub mod models;
pub mod withdraws;

use ethers::providers::{Provider, Ws};
use evm::address::Address;
use sqlx::PgPool;
use tonic::{transport::Channel, Request, Response, Status};

use crate::{
    base::{self, blockchain_server::Blockchain},
    contracts::treasury::Treasury,
    engine_base::engine_client::EngineClient,
};

use self::{
    deposits::DepositsBlockchainManagerController,
    withdraws::{models::WithdrawPermitRequest, WithdrawsBlockchainManagerController},
};

#[derive(Debug)]
pub struct BlockchainEngine {
    pub contract: Treasury<Provider<Ws>>,
    pub database: PgPool,
    pub deposits_controller: DepositsBlockchainManagerController,
    pub withdraws_controller: WithdrawsBlockchainManagerController,
    pub contract_key_address: Address,
    pub engine_client: EngineClient<Channel>,
}

#[tonic::async_trait]
impl Blockchain for BlockchainEngine {
    async fn withdraw(
        &self,
        request: Request<base::WithdrawPermitRequest>,
    ) -> Result<Response<base::WithdrawPermitResponse>, Status> {
        let request: WithdrawPermitRequest = request.into_inner().try_into()?;
        let response = self
            .withdraws_controller
            .withdraw(request, self.engine_client.to_owned())
            .await
            .map_err(|e| Status::aborted(e.to_string()))?;
        Ok(Response::new(base::WithdrawPermitResponse {
            signature: prefix_hex::encode(<[u8; 65]>::from(response)),
        }))
    }
}
