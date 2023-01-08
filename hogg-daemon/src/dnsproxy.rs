use std::{collections::HashMap, future::Future, time::Duration};

use anyhow::{anyhow, Result};
use chrono;
use hogg_common::{
    config::HoggConfig,
    dnslib::{BytePacketBuffer, DnsPacket},
};
use lazy_static::lazy_static;
use std::sync::Mutex;
use tokio::{net::UdpSocket, time::timeout};

type FA<R> = fn(String) -> R;
lazy_static! {
    static ref SCAN_CACHE: Mutex<HashMap<String, u64>> = Mutex::new(HashMap::new());
}

pub async fn dns_proxy_task(
    config: &HoggConfig,
    scan_function: FA<impl Future<Output = ()> + Send + 'static>,
) {
    if !config.dnsproxy.enabled {
        logs::warn!("DNS Proxy is disabled.");
        return;
    }

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

        let (mut resp, len) =
            match dispatch_incoming(req, len, config.dnsproxy.upstreams.clone()).await {
                Ok(res) => res,
                Err(e) => {
                    logs::error!("DNS Proxy failed to dispatch incoming packet: {}", e);
                    continue;
                }
            };

        if let Err(e) = socket.send_to(&resp.buf[..len], &src).await {
            logs::error!("DNS Proxy failed to send packet to downstream: {}", e)
        }

        if let Some(q) = DnsPacket::from_buffer(&mut resp)
            .unwrap()
            .questions
            .get(0)
        {
            let domain = q.name.to_string();
            let now = chrono::Utc::now().timestamp() as u64;
            let mut scan_cache = SCAN_CACHE.lock().unwrap();

            if let Some(last_scan) = scan_cache.get(&domain) {
                // Domain has been scanned before, check if TTL has expired
                if now - last_scan > config.scanner.cache_ttl.into() {
                    logs::debug!("Scanning {} [TTL expired]", domain);
                    scan_cache.insert(domain.clone(), now);
                    tokio::spawn(scan_function(domain));
                } else {
                    logs::debug!("Skipping {} [cache]", domain);
                }
            } else {
                // Domain has not been scanned before, scan it and add to cache
                logs::debug!("Scanning {} [first time scan]", domain);
                scan_cache.insert(domain.clone(), now);
                logs::trace!("Scan cache size: {:?}", scan_cache.len());
                tokio::spawn(scan_function(domain));
            }

            drop(scan_cache);
        }
    }
}

pub async fn dispatch_incoming(
    req: BytePacketBuffer,
    len: usize,
    upstreams: Vec<String>,
) -> Result<(BytePacketBuffer, usize)> {
    let buf = &req.buf[..len];

    for addr in upstreams {
        let socket = UdpSocket::bind(("0.0.0.0", 0)).await?;

        let data: Result<(BytePacketBuffer, usize)> = match timeout(Duration::from_secs(3), async {
            socket.send_to(buf, addr).await?;
            let mut res = BytePacketBuffer::new();
            let (len, _) = socket.recv_from(&mut res.buf).await?;
            Ok((res, len))
        })
        .await {
            Ok(data) => data,
            Err(_) => continue,
        };

        match data {
            Ok(data) => return Ok(data),
            Err(e) => return Err(e),
        };
    }

    Err(anyhow!(
        "DNS Proxy failed to dispatch incoming packet: no upstreams available"
    ))
}
