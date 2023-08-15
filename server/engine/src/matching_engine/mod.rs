use base::engine_server::Engine;
use sqlx::PgPool;
use tonic::{Request, Response, Status};
use uuid::Uuid;

use crate::{
    base::{self},
    database::managers::ValutsManager,
};

use self::models::{
    submit::SubmitRequest,
    transfer::{RevertTransferRequest, TransferRequest},
};

pub mod cancel;
pub mod matching_loop;
pub mod models;
pub mod submit;
pub mod transfer;

#[cfg(test)]
pub mod tests;

pub struct MatchingEngine {
    database: PgPool,
    fee_user_id: Uuid,
}

impl MatchingEngine {
    pub fn new(database: PgPool, fee_user_id: Uuid) -> Self {
        Self {
            database,
            fee_user_id,
        }
    }
}

#[tonic::async_trait]
impl Engine for MatchingEngine {
    async fn submit(
        &self,
        request: Request<base::SubmitRequest>,
    ) -> Result<Response<base::SubmitResponse>, Status> {
        let mut t = self
            .database
            .begin()
            .await
            .map_err(|e| Status::aborted(e.to_string()))?;

        let request: SubmitRequest = request.into_inner().try_into()?;

        let quote_fee_valut_id =
            ValutsManager::get_or_create(&mut t, self.fee_user_id, request.quote_asset_id)
                .await
                .map_err(|e| Status::invalid_argument(e.to_string()))?
                .id;
        let base_fee_valut_id =
            ValutsManager::get_or_create(&mut t, self.fee_user_id, request.base_asset_id)
                .await
                .map_err(|e| Status::invalid_argument(e.to_string()))?
                .id;

        Ok(Response::new(
            match submit::submit(request, quote_fee_valut_id, base_fee_valut_id, &mut t).await {
                Ok(r) => {
                    t.commit()
                        .await
                        .map_err(|e| Status::aborted(e.to_string()))?;
                    Ok(r)
                }
                Err(e) => {
                    t.rollback()
                        .await
                        .map_err(|e| Status::aborted(e.to_string()))?;
                    Err(e)
                }
            }
            .try_into()?,
        ))
    }

    async fn transfer(
        &self,
        request: Request<base::TransferRequest>,
    ) -> Result<Response<base::TransferResponse>, Status> {
        let mut t = self
            .database
            .begin()
            .await
            .map_err(|e| Status::aborted(e.to_string()))?;
        let request: TransferRequest = request.into_inner().try_into()?;
        let fee_valut_id = ValutsManager::get_or_create(&mut t, self.fee_user_id, request.asset_id)
            .await
            .map_err(|e| Status::invalid_argument(e.to_string()))?
            .id;
        Ok(Response::new(
            match transfer::transfer(
                &TransferRequest {
                    from_valut_id: request.from_valut_id,
                    to_valut_id: request.to_valut_id,
                    asset_id: request.asset_id,
                    amount: request.amount,
                    fee_fraction: request.fee_fraction,
                },
                &fee_valut_id,
                &mut t,
            )
            .await
            {
                Ok(r) => {
                    t.commit()
                        .await
                        .map_err(|e| Status::aborted(e.to_string()))?;
                    Ok(r)
                }
                Err(e) => {
                    t.rollback()
                        .await
                        .map_err(|e| Status::aborted(e.to_string()))?;
                    Err(e)
                }
            }
            .try_into()?,
        ))
    }

    async fn revert_transfer(
        &self,
        request: Request<base::RevertTransferRequest>,
    ) -> Result<Response<base::RevertTransferResponse>, Status> {
        let mut t = self
            .database
            .begin()
            .await
            .map_err(|e| Status::aborted(e.to_string()))?;
        let request: RevertTransferRequest = request.into_inner().try_into()?;
        Ok(Response::new(
            match transfer::revert_transfer(request.transfer_id, &mut t).await {
                Ok(r) => {
                    t.commit()
                        .await
                        .map_err(|e| Status::aborted(e.to_string()))?;
                    Ok(r)
                }
                Err(e) => {
                    t.rollback()
                        .await
                        .map_err(|e| Status::aborted(e.to_string()))?;
                    Err(e)
                }
            }
            .try_into()?,
        ))
    }

    async fn cancel(
        &self,
        request: Request<base::CancelRequest>,
    ) -> Result<Response<base::CancelResponse>, Status> {
        let mut t = self
            .database
            .begin()
            .await
            .map_err(|e| Status::aborted(e.to_string()))?;
        Ok(Response::new(
            match cancel::cancel(request.into_inner().try_into()?, &mut t).await {
                Ok(r) => {
                    t.commit()
                        .await
                        .map_err(|e| Status::aborted(e.to_string()))?;
                    Ok(r)
                }
                Err(e) => {
                    t.rollback()
                        .await
                        .map_err(|e| Status::aborted(e.to_string()))?;
                    Err(e)
                }
            }
            .try_into()?,
        ))
    }
}
