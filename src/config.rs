use serde_derive::{Serialize, Deserialize};
use std::path::Path;
use anyhow::Result;
use crate::scanner::Scanner;

use crate::scanner::scanners::GitCredsScanner;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    hijackers_path: String,

    pub gitcreds_enabled: bool
}

pub fn load_config(path: &Path) -> Result<Config> {
    Ok(toml::from_slice(std::fs::read(path)?.as_slice())?)
}

impl Config {
    pub fn get_hijackers_path(&self) -> &Path {
        Path::new(self.hijackers_path.as_str())
    }

    pub fn scanners_vec(&self) -> Vec<Box<dyn Scanner + Send + Sync>> {
        let mut result: Vec<Box<dyn Scanner + Send + Sync>> = vec![];
        if self.gitcreds_enabled { result.push(Box::new(GitCredsScanner {})) };

        result
    }
}
