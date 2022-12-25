use std::process::Stdio;
use serde::{Serialize, Deserialize};
use tokio::{process, io::{BufReader, AsyncBufReadExt}};
use anyhow::Result;

use crate::{config::Config, notifiers::scanner_notify, optimizers::ssladapter};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NucleiTreeInfo {
    pub name: String,
    pub author: Vec<String>,
    pub tags: Vec<String>,
    pub severity: String, // TODO: convert to enum if possible
    pub description: Option<String>,
    pub reference: Option<Vec<String>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct NucleiJsonOutput {
    pub template: String,
    #[serde(rename = "template-id")]
    pub template_id: String,
    pub info: NucleiTreeInfo,

    pub host: String,
    pub matched_at: Option<String>,

    // usually garbage
    #[serde(rename = "type")]
    pub type_: String,
    pub ip: Option<String>,
    pub timestamp: String,
    #[serde(rename = "curl-command")]
    pub curl_command: Option<String>,
    #[serde(rename = "matcher-status")]
    pub matcher_status: bool,
    #[serde(rename = "matched-line")]
    pub matched_line: Option<String>,
}

pub struct ServicesScanner {
    config: Config,
}

impl ServicesScanner {
    pub fn new(config_ctx: Config) -> Self {
        Self { config: config_ctx }
    }

    pub async fn scan(&self, target: String) -> Result<Vec<NucleiJsonOutput>> {
        logs::debug!("Scanning: {}", target);
        let mut answers: Vec<NucleiJsonOutput> = vec![];

        let target = match ssladapter::is_force_https(target.clone()).await {
            true => format!("https://{}", target),
            false => format!("http://{}", target),
        };

        let mut cmd = process::Command::new(self.config.nuclei.nuclei_path.as_str());
        cmd.stdout(Stdio::piped());
        cmd
            .arg("-u").arg(target.as_str())
            .arg("-config").arg(self.config.nuclei.config_file.as_str())
            .arg("--json");
            
            cmd.args(self.config.nuclei.cli_args.split(" "));

        let mut child = cmd.spawn()
            .expect("failed to spawn command");

        let stdout = child.stdout.take()
            .expect("child did not have a handle to stdout");

        let mut reader = BufReader::new(stdout).lines();

        tokio::spawn(async move {
            child.wait().await.expect("child process encountered an error");
        });

        while let Some(line) = reader.next_line().await? {
            // logs::debug!("Received line from nuclei stdout reader: {}", line.trim());
            let json: NucleiJsonOutput = match serde_json::from_str(line.trim()) {
                Ok(json) => json,
                Err(e) => {
                    logs::debug!("Non-json line received from nuclei stdout reader: {}", e);
                    continue;
                }
            };

            logs::debug!("New nuclei profits: {:#?}", json);
            answers.push(json.clone());

            let c = self.config.clone();
            tokio::spawn(async move { 
                scanner_notify(json, &c).await.unwrap();
            });
        }

        Ok(answers)
    } 
}
