use std::{future::Future, time::Duration};

use hogg_common::{config::HoggConfig, dnslib::{BytePacketBuffer, DnsPacket}};
use tokio::{net::UdpSocket, time::timeout};
use anyhow::{anyhow, Result};


type FA<R> = fn(String) -> R;


pub async fn dns_proxy_task(
    config: &HoggConfig, 
    scan_function: FA<impl Future<Output = ()> + Send + 'static>
) {
    let socket = match UdpSocket::bind(config.dnsproxy.bind.clone()).await {
        Ok(socket) => socket,
        Err(e) => {
            logs::error!("DNS Proxy failed to start: {}", e);
            return;
        }
    };

    loop {
        let mut req = BytePacketBuffer::new();
        
        let (len, src) = match socket.recv_from(&mut req.buf).await {
            Ok((len, src)) => (len, src),
            Err(_) => continue,
        };

        let mut upstream_response = match dispatch_incoming(req, len, config.dnsproxy.upstreams.clone()).await {
            Ok(res) => res,
            Err(e) => {
                logs::error!("DNS Proxy failed to dispatch incoming packet: {}", e);
                continue;
            }
        };

        if let Err(e) = socket.send_to(&upstream_response.buf, &src).await {
            logs::error!(
                "DNS Proxy failed to send packet to downstream: {}",
                e
            )
        }

        if let Some(q) = DnsPacket::from_buffer(&mut upstream_response).unwrap()
            .questions
            .get(0)
        {
            tokio::spawn(scan_function(q.name.to_string()));
        }
    }
    
    
}


pub async fn dispatch_incoming(
    req: BytePacketBuffer, 
    len: usize, 
    upstreams: Vec<String>
) -> Result<BytePacketBuffer> {
    let buf = &req.buf[..len];

    for addr in upstreams {
        let socket = UdpSocket::bind(("0.0.0.0", 0)).await?;

        let data: Result<BytePacketBuffer> = match timeout(Duration::from_secs(3), async {
            socket.send_to(buf, addr).await?;
            let mut res = BytePacketBuffer::new();
            socket.recv_from(&mut res.buf).await?;
            Ok(res)
        }).await {
            Ok(data) => data,
            Err(_) => continue,
        };

        match data {
            Ok(data) => return Ok(data),
            Err(e) => return Err(e),
        };
    }

    Err(anyhow!("DNS Proxy failed to dispatch incoming packet: no upstreams available"))
}