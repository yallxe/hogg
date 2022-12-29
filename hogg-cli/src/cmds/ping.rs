use anyhow::Result;
use hogg_grpc::grpc;

pub async fn run() -> Result<()> {
    let mut grpc = grpc::connect_grpc_client().await?;

    let ping_request = grpc::PingRequest {
        message: "Hello from hogg-cli".to_string(),
    };

    let ping_response = grpc.ping(ping_request).await?;

    logs::info!("Received pong: {:?}", ping_response);

    Ok(())
}
