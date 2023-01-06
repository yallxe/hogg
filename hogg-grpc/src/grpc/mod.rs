pub mod services;

pub mod daemon_proto {
    tonic::include_proto!("daemon");
}

pub use daemon_proto::daemon_server::{Daemon, DaemonServer};
pub use daemon_proto::{PingRequest, PingResponse, ReloadDatabaseRequest, ReloadDatabaseResponse};
use tonic::transport::Channel;

use self::daemon_proto::daemon_client::DaemonClient;

#[derive(Debug, err_derive::Error)]
pub enum Error {
    #[error(display = "GRPC Transport Error")]
    GrpcTransportError(#[error(source)] tonic::transport::Error),
}

pub fn tokio_serve_hogg_grpc<F: Send + Sync + 'static + Fn() -> ()>(db_reload: F) -> Result<tokio::task::JoinHandle<Result<(), Error>>, Error> {
    Ok(tokio::spawn(async move {
        let addr = "[::1]:1396".parse().unwrap();
        let daemon_health = services::DaemonService::new(db_reload);

        tonic::transport::Server::builder()
            .add_service(DaemonServer::new(daemon_health))
            .serve(addr)
            .await
            .map_err(Error::GrpcTransportError)
    }))
}

pub async fn connect_grpc_client() -> Result<DaemonClient<Channel>, Error> {
    Ok(DaemonClient::connect("http://[::1]:1396").await?)
}
