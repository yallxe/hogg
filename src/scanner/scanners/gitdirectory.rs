use crate::scanner::{
    scanners::{default_http_client, Scanner},
    ScanAnswer,
};
use async_trait::async_trait;

pub struct GitDirectoryScanner {}

#[async_trait]
impl Scanner for GitDirectoryScanner {
    fn name(&self) -> String {
        "Git Directory Leak".to_string()
    }

    async fn process(&self, target: &str) -> anyhow::Result<Vec<ScanAnswer>> {
        let client = default_http_client().build()?;
        let response = client
            .get(reqwest::Url::parse(target)?.join("/.git/config")?)
            .send()
            .await?;

        let url = response.url().to_string();
        logs::debug!("{}", url);
        let mut answers: Vec<ScanAnswer> = vec![];

        if response.text().await.unwrap_or_default().contains("[core]") {
            answers.push(ScanAnswer {
                full_url: url,
                detection_name: self.name(),
            })
        }

        Ok(answers)
    }
}
