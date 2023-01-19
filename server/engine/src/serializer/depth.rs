use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

use crate::base::{DrainOrderBookRequest, DrainOrderBookResponse};

type DrainOrderBookStream = ReceiverStream<Result<DrainOrderBookResponse, Status>>;

async fn drain_order_book(
    _request: Request<DrainOrderBookRequest>,
) -> Result<Response<DrainOrderBookStream>, Status> {
    unimplemented!()
}
