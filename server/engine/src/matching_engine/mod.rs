use base::{engine_server::Engine, CancelRequest, CancelResponse, SubmitRequest, SubmitResponse};
use sqlx::{Pool, Postgres};
use tonic::{Request, Response, Status};

use crate::base;
pub struct MatchingEngine {
    database: Pool<Postgres>,
}

impl MatchingEngine {
    pub fn new(database: Pool<Postgres>) -> Self {
        Self { database }
    }
}

#[tonic::async_trait]
impl Engine for MatchingEngine {
    async fn submit(
        &self,
        request: Request<SubmitRequest>,
    ) -> Result<Response<SubmitResponse>, Status> {
        Ok(Response::new(SubmitResponse {}))
    }

    async fn cancel(
        &self,
        request: Request<CancelRequest>,
    ) -> Result<Response<CancelResponse>, Status> {
        Ok(Response::new(CancelResponse {}))
    }
}
