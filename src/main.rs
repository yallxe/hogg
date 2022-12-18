use std::sync::Arc;

use crate::sniffers::Sniffer;
use anyhow::Result;
use env::get_hogg_dir;
use sniffers::dnsproxy::DnsProxySniffer;

mod config;
mod env;
mod sniffers;
mod scanner;

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

    let config_path = get_hogg_dir();
    std::env::set_current_dir(&config_path)?;

    let config = match config::load_config(
        std::path::Path::new("config.toml")
    ) {
        Ok(config) => config,
        Err(e) => exit!("Failed to load config.toml: {}", e),
    };

    let scanner = Arc::new(scanner::ServicesScanner::new(&config));

    if let Ok(mut hijacker) = DnsProxySniffer::new() {
        let scanner = scanner.clone();
        tokio::spawn(async move {
            logs::info!("{} is starting up...", hijacker.name());
            hijacker.run(&scanner).await;
        });
    }

    loop { } // required so the main thread doesn't exit
}
