use std::{sync::Arc, path::Path};

use crate::sniffers::Sniffer;
use anyhow::Result;
use env::get_hogg_dir;
use sniffers::dnsproxy::DnsProxySniffer;
use include_dir::{include_dir, Dir};

mod config;
mod env;
mod sniffers;
mod scanner;
mod notifiers;
mod optimizers;

#[macro_export]
macro_rules! exit {
    ($($arg:tt)*) => {
        {
            logs::error!($($arg)*);
            std::process::exit(1)
        }
    };
}

static CONFIG_TEMPLATE: Dir<'_> = include_dir!("hogg-config");

fn create_config_template() {
    for entry in CONFIG_TEMPLATE.find("*.*").unwrap() {
        let path = entry.as_file().unwrap().path();
        match path.parent() {
            Some(_) => {
                std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            },
            None => {},
        };
        
        if !path.exists() {
            std::fs::write(path, entry.as_file().unwrap().contents()).unwrap();
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    logs::Logs::new().init();

    let config_path = get_hogg_dir();
    if !Path::new(&config_path).exists() {
        logs::info!("Creating config directory");
        std::fs::create_dir_all(&config_path)?;
    }
    std::env::set_current_dir(&config_path)?;

    create_config_template();

    let config = match config::load_config(
        std::path::Path::new("config.toml")
    ) {
        Ok(config) => config,
        Err(e) => exit!("Failed to load config.toml: {}", e),
    };

    let scanner = Arc::new(scanner::ServicesScanner::new(config));

    if let Ok(mut hijacker) = DnsProxySniffer::new() {
        let scanner = scanner.clone();
        tokio::spawn(async move {
            logs::info!("{} is starting up...", hijacker.name());
            hijacker.run(&scanner).await;
        });
    }

    loop { } // required so the main thread doesn't exit
}
