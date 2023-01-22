use tonic::{Request, Response, Status};

use crate::base::{CancelRequest, CancelResponse};

async fn cancel(_request: Request<CancelRequest>) -> Result<Response<CancelResponse>, Status> {
    unimplemented!()
}
