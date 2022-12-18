use anyhow::Result;
use serde_derive::{Deserialize, Serialize};
use std::path::Path;

fn default_nuclei_path() -> String {
    "nuclei".to_string()
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NucleiConfig {
    #[serde(default="default_nuclei_path")]
    pub nuclei_path: String,
    pub config_file: String,
    #[serde(default)]
    pub cli_args: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub nuclei: NucleiConfig,
}

pub fn load_config(path: &Path) -> Result<Config> {
    Ok(toml::from_slice(std::fs::read(path)?.as_slice())?)
}
