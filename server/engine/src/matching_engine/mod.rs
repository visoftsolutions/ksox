use base::engine_server::Engine;
use fraction::Fraction;
use sqlx::PgPool;
use tonic::{Request, Response, Status};

use crate::base;

pub mod burn;
pub mod cancel;
pub mod matching_loop;
pub mod mint;
pub mod models;
pub mod submit;
pub mod transfer;

#[cfg(test)]
pub mod tests;

pub struct MatchingEngine {
    accuracy: Fraction,
    database: PgPool,
}

impl MatchingEngine {
    pub fn new(database: PgPool, accuracy: Fraction) -> Self {
        Self { accuracy, database }
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
        Ok(Response::new(
            match submit::submit(
                request.into_inner().try_into()?,
                &mut t,
                self.accuracy.clone(),
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

    async fn transfer(
        &self,
        request: Request<base::TransferRequest>,
    ) -> Result<Response<base::TransferResponse>, Status> {
        let mut t = self
            .database
            .begin()
            .await
            .map_err(|e| Status::aborted(e.to_string()))?;
        Ok(Response::new(
            match transfer::transfer(request.into_inner().try_into()?, &mut t).await {
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

    async fn mint(
        &self,
        request: Request<base::MintRequest>,
    ) -> Result<Response<base::MintResponse>, Status> {
        let mut t = self
            .database
            .begin()
            .await
            .map_err(|e| Status::aborted(e.to_string()))?;
        Ok(Response::new(
            match mint::mint(request.into_inner().try_into()?, &mut t).await {
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

    async fn burn(
        &self,
        request: Request<base::BurnRequest>,
    ) -> Result<Response<base::BurnResponse>, Status> {
        let mut t = self
            .database
            .begin()
            .await
            .map_err(|e| Status::aborted(e.to_string()))?;
        Ok(Response::new(
            match burn::burn(request.into_inner().try_into()?, &mut t).await {
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
