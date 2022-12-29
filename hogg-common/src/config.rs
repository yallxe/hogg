use anyhow::Result;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HoggConfig {
    pub dnsproxy: DnsProxyConfig,
    pub daemon: DaemonConfig,
    pub scanner: ScannerConfig,
    pub database: DatabaseConfig,

    #[serde(skip)]
    pub _file: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DnsProxyConfig {
    #[serde(default)]
    pub enabled: bool,
    pub bind: String,
    pub upstreams: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DaemonConfig {
    pub api: DaemonApiConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DaemonApiConfig {
    pub enabled: bool,
    pub bind: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "snake_case", deserialize = "kebab-case"))]
pub struct ScannerConfig {
    pub nuclei: ScannerNucleiConfig,
    pub check_force_ssl: bool,
    pub cache_ttl: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "snake_case", deserialize = "kebab-case"))]
pub struct ScannerNucleiConfig {
    pub nuclei_executable: String,
    pub cli_args: Vec<String>,
    pub using_community_templates: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DatabaseConfig {
    pub save_detections: bool,
    pub detections_limiter_enabled: bool,
    pub max_detections: usize
}

impl HoggConfig {
    pub fn from_file(path: &str) -> Result<Self> {
        let config = std::fs::read_to_string(path)?;
        let mut config: HoggConfig = toml::from_str(&config)?;
        config._file = std::fs::canonicalize(path)?.to_str().unwrap().to_string();
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let config = toml::to_string_pretty(&self)?;
        std::fs::write(&self._file, config)?;
        Ok(())
    }
}

pub fn create_config_template(dir: include_dir::Dir<'_>) {
    for entry in dir.find("*.*").unwrap() {
        let path = entry.as_file().unwrap().path();

        if let Some(_) = path.parent() {
            std::fs::create_dir_all(path.parent().unwrap()).unwrap()
        }

        if !path.exists() {
            std::fs::write(path, entry.as_file().unwrap().contents()).unwrap();
        }
    }
}
