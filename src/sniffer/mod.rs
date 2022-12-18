use crate::scanner::ServicesScanner;
use async_trait::async_trait;

#[async_trait]
pub trait Sniffer {
    fn name(&self) -> String; // Returns name of sniffer (for example, "DNS Proxy Sniffer")
    async fn run(&mut self, scanner_ctx: &ServicesScanner); // As sniffing is "forever" process, this function returns nothing and is called as a task.
}

macro_rules! get_configuration {
    ($hjcfg_path:tt, $cfgt:ty) => {
        {
            let configuration: $cfgt = toml::from_slice(
                std::fs::read(Path::new("sniffers").join($hjcfg_path))?.as_slice(),
            )?;
            if !configuration.enabled {
                return Err(anyhow!("Sniffer is disabled"));
            }
    
            Ok(configuration)
        }
    };
}

pub mod dnsproxy;
