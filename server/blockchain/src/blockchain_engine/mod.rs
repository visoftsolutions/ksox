pub mod deposits;
pub mod models;
pub mod valuts;
pub mod withdraws;

use ethers::{
    prelude::{k256::ecdsa::SigningKey, SignerMiddleware},
    providers::{Provider, Ws},
    signers::Wallet,
};
use evm::txhash::TxHash;
use fraction::Fraction;
use sqlx::PgPool;
use tonic::{Request, Response, Status};

use crate::{
    base::{self, blockchain_server::Blockchain},
    contracts::treasury::Treasury,
    database::projections::withdraw::WithdrawInsert,
};

use self::{
    deposits::DepositsBlockchainManagerController,
    valuts::ValutsBlockchainManagerController,
    withdraws::{models::WithdrawRequest, WithdrawsBlockchainManagerController},
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
        let request: WithdrawRequest = request.into_inner().try_into()?;
        tracing::info!("{:?}", request);
        self.withdraws_controller
            .withdraw(WithdrawInsert {
                maker_address: request.maker_address,
                taker_address: request.taker_address,
                asset_address: request.asset_address,
                tx_hash: TxHash::default(),
                amount: request.amount,
                confirmations: Fraction::default(),
            })
            .await
            .map_err(|e| Status::aborted(e.to_string()))?;
        Ok(Response::new(base::WithdrawResponse {}))
    }
}
