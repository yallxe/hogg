use tonic::{Request, Response, Status};

use crate::grpc::{Daemon, PingRequest, PingResponse, ReloadDatabaseRequest, ReloadDatabaseResponse};

#[derive(Debug)]
pub struct DaemonService<F: Send + Sync + 'static + Fn() -> ()> {
    pub db_reload: F,
}

impl<F: Send + Sync + 'static + Fn() -> ()> DaemonService<F> {
    pub fn new(db_reload: F) -> Self {
        Self {
            db_reload,
        }
    }
}

#[tonic::async_trait]
impl<F: Send + Sync + 'static + Fn() -> ()> Daemon for DaemonService<F> {
    async fn ping(&self, request: Request<PingRequest>) -> Result<Response<PingResponse>, Status> {
        logs::trace!("Got ping request: {:?}", request);

        let reply = PingResponse {
            message: "Hello from the other side!".into(),
        };

        Ok(Response::new(reply))
    }

    async fn reload_database(
        &self,
        _request: Request<ReloadDatabaseRequest>,
    ) -> Result<Response<ReloadDatabaseResponse>, Status> {
        (self.db_reload)();

        Ok(Response::new(ReloadDatabaseResponse {
            success: true,
            error: None
        }))
    }
}
