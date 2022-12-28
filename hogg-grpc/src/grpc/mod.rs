pub mod services;

pub mod daemon_proto {
    tonic::include_proto!("daemon");
}

pub use daemon_proto::daemon_health_server::{DaemonHealth, DaemonHealthServer};
pub use daemon_proto::{PingRequest, PingResponse};


#[derive(Debug, err_derive::Error)]
pub enum Error {
    #[error(display = "GRPC Transport Error")]
    GrpcTransportError(#[error(source)] tonic::transport::Error),
}

pub fn tokio_serve_hogg_grpc() -> Result<tokio::task::JoinHandle<Result<(), Error>>, Error> {
    Ok(tokio::spawn(async move {
        let addr = "[::1]:1396".parse().unwrap();
        let daemon_health = services::DaemonHealthService::default();

        tonic::transport::Server::builder()
            .add_service(DaemonHealthServer::new(daemon_health))
            .serve(addr)
            .await
            .map_err(Error::GrpcTransportError)
    }))
}
