use std::{net::SocketAddr, time::Duration};
use async_trait::async_trait;
use crate::hijackers::Hijacker;
use crate::scanner::ServicesScanner;
use anyhow::{anyhow, Result};
use hogg::{BytePacketBuffer, DnsPacket};
use logs::error;
use tokio::{net::UdpSocket, time::timeout};

pub struct DnsProxyHijacker {
    pub bind: SocketAddr,
    pub upstreams: Vec<SocketAddr>,

    socket: Option<UdpSocket>,
}

impl DnsProxyHijacker {
    pub fn new(bind: SocketAddr, upstreams: Vec<SocketAddr>) -> Self {
        Self {
            bind, upstreams,
            socket: None,
        }
    }

    pub async fn proxy_worker(&self, scanner_ctx: &ServicesScanner) -> Result<()> {
        if self.socket.is_none() {
            return Err(anyhow!("Socket wasn't established, but worker function of DNS Proxy Hijacker was called"));
        }
        let socket = self.socket.as_ref().unwrap();

        let mut req = BytePacketBuffer::new();

        let (len, src) = match socket.recv_from(&mut req.buf).await {
            Ok((len, src)) => (len, src),
            Err(e) => {
                error!("DnsProxyHijacker failed to recv_from with downstream: {}", e);
                return Ok(());
            }
        };

        let mut upstream_response = match self.dispatch(req, len).await {
            Ok(res) => res,
            Err(e) => {
                error!("DnsProxyHijacker failed to dispatch packet: {}", e);
                return Ok(());
            }
        };

        if let Some(q) = DnsPacket::from_buffer(&mut upstream_response)?.questions.get(0) {
            scanner_ctx.scan(format!("http://{}", q.name)).await?;
            scanner_ctx.scan(format!("https://{}", q.name)).await?;
        }

        if let Err(e) = socket.send_to(&upstream_response.buf, &src).await {
            error!("DnsProxyHijacker failed to send packet to downstream: {}", e)
        }

        Ok(())
    }

    async fn dispatch(&self, req: BytePacketBuffer, len: usize) -> Result<BytePacketBuffer> {
        let buf = &req.buf[..len];

        for addr in self.upstreams.iter() {
            let socket = UdpSocket::bind(("0.0.0.0", 0)).await?;

            let data: Result<BytePacketBuffer> = timeout(Duration::from_secs(3), async {
                socket.send_to(buf, addr).await?;
                let mut res = BytePacketBuffer::new();
                socket.recv_from(&mut res.buf).await?;
                Ok(res)
            }).await?;

            match data {
                Ok(data) => return Ok(data),
                Err(e) => return Err(e)
            };
        }

        Err(anyhow!("No upstreams found."))
    }
}

#[async_trait]
impl Hijacker for DnsProxyHijacker {
    async fn run(&mut self, scanner_ctx: &ServicesScanner) {
        self.socket = match UdpSocket::bind(self.bind).await {
            Ok(socket) => Some(socket),
            Err(e) => {
                error!("DnsProxyHijacked failed to start: {}", e);
                return;
            }
        };
        loop {
            if let Err(e) = self.proxy_worker(scanner_ctx).await {
                error!("DnsProxyHijacker crashed due to: {}", e);
                break;
            }
        }
    }
}