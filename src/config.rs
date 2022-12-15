use crate::env::get_hogg_dir;
use anyhow::Result;
use serde_derive::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    hijackers_path: String
}

pub fn load_config(path: &Path) -> Result<Config> {
    Ok(toml::from_slice(std::fs::read(path)?.as_slice())?)
}

impl Config {
    pub fn get_hijackers_path(&self) -> PathBuf {
        Path::new(get_hogg_dir().as_str()).join(self.hijackers_path.as_str())
    }
}
