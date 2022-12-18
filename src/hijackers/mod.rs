use crate::scanner::ServicesScanner;
use async_trait::async_trait;

#[async_trait]
pub trait Hijacker {
    fn name(&self) -> String; // Returns name of hijacker (for example, "DNS Proxy Hijacker")
    async fn run(&mut self, scanner_ctx: &ServicesScanner); // As hijacking is "forever" process, this function returns nothing and is called as a task.
}

macro_rules! new_hijacker {
    ($hjcfg_path:tt) => {
        {
            let configuration: DnsProxyHijackerConfiguration = toml::from_slice(
                std::fs::read(Path::new("hijackers").join($hjcfg_path))?.as_slice(),
            )?;
            if !configuration.enabled {
                return Err(anyhow!("Hijacker is disabled"));
            }
    
            Ok(Self {
                configuration,
                socket: None,
            })
        }
    };
}

pub mod dnsproxy;
