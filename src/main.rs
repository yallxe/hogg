use crate::hijackers::Hijacker;
use anyhow::Result;
use env::get_hogg_dir;
use hijackers::dnsproxy::DnsProxyHijacker;

mod config;
mod env;
mod hijackers;
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

    let config = match config::load_config(
        std::path::Path::new(&config_path)
            .join("config.toml")
            .as_path(),
    ) {
        Ok(config) => config,
        Err(e) => exit!("Failed to load {}/config.toml: {}", config_path, e),
    };

    let scanner = scanner::ServicesScanner::new(config.scanners_vec());

    if let Ok(mut hijacker) = DnsProxyHijacker::new(&config) {
        logs::info!("{} has been started", hijacker.name());
        hijacker.run(&scanner).await;
        tokio::spawn(async move {
            hijacker.run(&scanner).await;
        });
    }

    Ok(())
}
