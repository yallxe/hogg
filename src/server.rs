use std::{net::SocketAddr, time::Duration};
use anyhow::Result;
use hogg::BytePacketBuffer;
use logs::error;
use tokio::{net::UdpSocket, time::timeout};

use crate::exit;

pub struct ServerConfig {
    pub bind: SocketAddr,
    pub upstream: Vec<SocketAddr>,
}

pub async fn run_server(server_config: &ServerConfig) {
    let socket = match UdpSocket::bind(server_config.bind).await {
        Ok(socket) => socket,
        Err(e) => {
            exit!("Failed to bind to UDP socket: {}", e);
        }
    };

    loop {
        let mut req = BytePacketBuffer::new();

        let (len, src) = match socket.recv_from(&mut req.buf).await {
            Ok((len, src)) => (len, src),
            Err(e) => {
                error!("Failed to receive UDP packet: {}", e);
                continue;
            }
        };

        let res = match dispatch(req, len, server_config).await {
            Ok(res) => res,
            Err(e) => {
                error!("Failed to dispatch UDP packet: {}", e);
                continue;
            }
        };

        if let Err(e) = socket.send_to(&res, &src).await {
            error!("Failed to send UDP packet: {}", e);
        }
    }
}

pub async fn dispatch(req: BytePacketBuffer, len: usize, server_config: &ServerConfig) -> Result<Vec<u8>> {
    let buf = &req.buf[..len];

    for addr in server_config.upstream.iter() {
        let socket = UdpSocket::bind(("0.0.0.0", 0)).await?;

        let data: Result<Vec<u8>> = timeout(Duration::from_millis(2000), async {
            socket.send_to(buf, addr).await?;
            let mut res = [0u8; 512];
            let (recv_len, _) = socket.recv_from(&mut res).await?;
            Ok(res[..recv_len].to_vec())
        }).await?;

        match data {
            Ok(data) => return Ok(data),
            Err(e) => error!("Failed to send proxy DNS packet to upstream {}: {}", addr, e),
        }
    }

    Ok(vec![])
}