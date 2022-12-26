use std::path::Path;

use anyhow::Result;
use dnsproxy::dns_proxy_task;
use include_dir::{include_dir, Dir};

use hogg_common::{
    config::{self, HoggConfig},
    env,
};

mod dnsproxy;
mod nuclei;

static CONFIG_TEMPLATE: Dir<'_> = include_dir!("resources/config-template");
static mut CONFIG: Option<HoggConfig> = None;

async fn scan_function(domain: String) {
    logs::info!("Scanning {}", domain);
    let config = unsafe { CONFIG.clone().unwrap() };
    let scan = nuclei::scan_with_nuclei(domain, &config).await.unwrap();
    logs::info!("Scan finished and found {} results", scan.len());
}

#[tokio::main]
async fn main() -> Result<()> {
    logs::Logs::new().color(true).init();

    let config_path = env::get_hogg_dir();
    if !Path::new(&config_path).exists() {
        logs::info!("Creating config directory");
        std::fs::create_dir_all(&config_path)?;
    }
    std::env::set_current_dir(&config_path)?; // not quite pretty technique

    config::create_config_template(CONFIG_TEMPLATE.clone());
    let config = config::HoggConfig::from_file("hogg.toml").unwrap();
    unsafe {
        CONFIG = Some(config.clone());
    }

    dns_proxy_task(&config, scan_function).await;
    Ok(())
}
