pub mod deposits;
pub mod models;
pub mod withdraws;

use tonic::{transport::Channel, Request, Response, Status};

use crate::{
    base::{self, blockchain_server::Blockchain},
    engine_base::engine_client::EngineClient,
};

use self::{
    deposits::DepositsBlockchainManagerController,
    models::{WithdrawPermitRequest, WithdrawPermitResponse},
    withdraws::WithdrawsBlockchainManagerController,
};

#[derive(Debug)]
pub struct BlockchainEngine {
    pub deposits_controller: DepositsBlockchainManagerController,
    pub withdraws_controller: WithdrawsBlockchainManagerController,
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
        Ok(Response::new(
            WithdrawPermitResponse {
                signature: response,
            }
            .into(),
        ))
    }
}
