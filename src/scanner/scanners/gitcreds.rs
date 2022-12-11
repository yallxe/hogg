use crate::scanner::{scanners::{Scanner, default_http_client}, ScanAnswer};
use async_trait::async_trait;

pub struct GitCredsScanner {}

#[async_trait]
impl Scanner for GitCredsScanner {
    fn name(&self) -> String { "Git Credentials Leak".to_string() }

    async fn process(&self, target: &String) -> anyhow::Result<Vec<ScanAnswer>> {
        let client = default_http_client().build()?;
        let response = client.get(reqwest::Url::parse(target.as_str())?.join("/.git/config")?)
            .send().await?;

        let url = response.url().to_string();
        logs::debug!("{}", url);
        let mut answers: Vec<ScanAnswer> = vec![];

        if response.text().await.unwrap_or(String::new()).starts_with("[core]") {
            answers.push(ScanAnswer { full_url: url, detection_name: self.name() })
        }

        Ok(answers)
    }
}