use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

use crate::base::{DrainTradesRequest, DrainTradesResponse};

type DrainTradesStream = ReceiverStream<Result<DrainTradesResponse, Status>>;

async fn drain_trades(
    _request: Request<DrainTradesRequest>,
) -> Result<Response<DrainTradesStream>, Status> {
    unimplemented!()
}
