use anyhow::Result;
use hijackers::dnsproxy::DnsProxyHijacker;
use crate::hijackers::Hijacker;

mod scanner;
mod hijackers;
mod config;

#[macro_export]
macro_rules! exit {
    ($($arg:tt)*) => {
        {
            logs::error!($($arg)*);
            std::process::exit(1)
        }
    };
}

#[tokio::main]
async fn main() -> Result<()> {
    logs::Logs::new().init();
    let config = config::load_config(std::path::Path::new("config.toml"))?;

    let scanner = scanner::ServicesScanner::new(config.scanners_vec());
    DnsProxyHijacker::new(&config)?.run(&scanner).await;

    Ok(())
}
