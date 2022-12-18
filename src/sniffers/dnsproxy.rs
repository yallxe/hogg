use crate::sniffer::Sniffer;
use crate::scanner::ServicesScanner;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use hogg::{BytePacketBuffer, DnsPacket};
use logs::error;
use serde_derive::{Deserialize, Serialize};
use std::{time::Duration, path::Path};
use tokio::{net::UdpSocket, time::timeout};

#[derive(Serialize, Deserialize)]
pub struct DnsProxySnifferConfiguration {
    pub enabled: bool,
    pub bind: String,
    pub upstreams: Vec<String>,
}

pub struct DnsProxySniffer {
    configuration: DnsProxySnifferConfiguration,

    socket: Option<UdpSocket>,
}

impl DnsProxySniffer {
    pub fn new() -> Result<Self> {
        match get_configuration!("dnsproxy.toml", DnsProxySnifferConfiguration) {
            Ok(configuration) => Ok(Self {
                configuration,
                socket: None,
            }),
            Err(e) => Err(e),
        }
    }

    pub async fn proxy_worker(&self, scanner_ctx: &ServicesScanner) -> Result<()> {
        if self.socket.is_none() {
            return Err(anyhow!(
                "Socket wasn't established, but worker function of DNS Proxy Sniffer was called"
            ));
        }
        let socket = self.socket.as_ref().unwrap();

        let mut req = BytePacketBuffer::new();

        let (len, src) = match socket.recv_from(&mut req.buf).await {
            Ok((len, src)) => (len, src),
            Err(e) => {
                error!(
                    "DnsProxySniffer failed to recv_from with downstream: {}",
                    e
                );
                return Ok(());
            }
        };

        let mut upstream_response = match self.dispatch(req, len).await {
            Ok(res) => res,
            Err(e) => {
                error!("DnsProxySniffer failed to dispatch packet: {}", e);
                return Ok(());
            }
        };

        if let Err(e) = socket.send_to(&upstream_response.buf, &src).await {
            error!(
                "DnsProxySniffer failed to send packet to downstream: {}",
                e
            )
        }

        if let Some(q) = DnsPacket::from_buffer(&mut upstream_response)?
            .questions
            .get(0)
        {
            if let Err(e) = scanner_ctx.scan(q.name.to_string()).await {
                error!("Failed to scan: {}", e);
            }
        }

        Ok(())
    }

    async fn dispatch(&self, req: BytePacketBuffer, len: usize) -> Result<BytePacketBuffer> {
        if self.configuration.upstreams.is_empty() {
            return Err(anyhow!("No upstreams found."));
        }

        let buf = &req.buf[..len];

        for addr in self.configuration.upstreams.iter() {
            logs::debug!("Trying {}", addr);
            let socket = UdpSocket::bind(("0.0.0.0", 0)).await?;

            let data: Result<BytePacketBuffer> = match timeout(Duration::from_secs(3), async {
                socket.send_to(buf, addr).await?;
                let mut res = BytePacketBuffer::new();
                socket.recv_from(&mut res.buf).await?;
                Ok(res)
            })
            .await
            {
                Ok(data) => data,
                Err(_) => continue,
            };

            match data {
                Ok(data) => return Ok(data),
                Err(e) => return Err(e),
            };
        }

        Err(anyhow!("No upstreams responded."))
    }
}

#[async_trait]
impl Sniffer for DnsProxySniffer {
    fn name(&self) -> String {
        "DNS Proxy Sniffer".to_string()
    }

    async fn run(&mut self, scanner_ctx: &ServicesScanner) {
        self.socket = match UdpSocket::bind(self.configuration.bind.clone()).await {
            Ok(socket) => Some(socket),
            Err(e) => {
                error!("DnsProxySniffer failed to start: {}", e);
                return;
            }
        };
        loop {
            if let Err(e) = self.proxy_worker(scanner_ctx).await {
                error!("DnsProxySniffer crashed due to: {}", e);
                break;
            }
        }
    }
}
