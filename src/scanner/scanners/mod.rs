use crate::scanner::ScanAnswer;
use anyhow::Result;
use async_trait::async_trait;

const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.0.0 Safari/537.36";

#[async_trait]
pub trait Scanner {
    fn name(&self) -> String;
    async fn process(&self, target: &str) -> Result<Vec<ScanAnswer>>;
}

pub fn default_http_client() -> reqwest::ClientBuilder {
    reqwest::Client::builder()
        .user_agent(USER_AGENT)
        .cookie_store(true)
}

pub mod gitdirectory;

pub use gitdirectory::GitDirectoryScanner;
