use tonic::{Request, Response, Status};

use crate::grpc::{DaemonHealth, PingRequest, PingResponse};

#[derive(Debug, Default)]
pub struct DaemonHealthService {}

#[tonic::async_trait]
impl DaemonHealth for DaemonHealthService {
    async fn ping(
        &self,
        request: Request<PingRequest>,
    ) -> Result<Response<PingResponse>, Status> {
        logs::trace!("Got ping request: {:?}", request);

        let reply = PingResponse {
            message: "Hello from the other side!".into(),
        };

        Ok(Response::new(reply))
    }
}