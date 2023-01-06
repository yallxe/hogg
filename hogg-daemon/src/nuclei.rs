use std::{path::Path, vec};

use anyhow::Result;
use hogg_common::{config::HoggConfig, db::HoggDatabase, ssladapter};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
};

use serde::{Deserialize, Serialize};

use crate::notifications;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct NucleiTreeInfo {
    pub name: String,
    pub author: Vec<String>,
    pub tags: Vec<String>,
    pub severity: String, // TODO: convert to enum if possible
    pub description: Option<String>,
    pub reference: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

impl PartialEq for NucleiJsonOutput {
    fn eq(&self, other: &Self) -> bool {
        self.template_id == other.template_id && 
        self.matched_at == other.matched_at && 
        self.host == other.host && 
        self.info == other.info
    }
}

impl Eq for NucleiJsonOutput {}

static mut DATABASE: Option<HoggDatabase<NucleiJsonOutput>> = None;
pub const DATABASE_FILENAME: &'static str = ".hoggdb.json";

pub fn load_database(config: &HoggConfig) {
    logs::debug!("(Re)loading nuclei database");
    unsafe {
        DATABASE = Some(
            HoggDatabase::from_file(
                Path::new(&config._file)
                    .parent()
                    .unwrap()
                    .join(DATABASE_FILENAME)
                    .as_path()
                    .to_str()
                    .unwrap()
                    .to_string(),
                config.clone(),
            )
            .unwrap(),
        );
    }
}

pub async fn scan_with_nuclei(
    domain: String,
    config: &HoggConfig,
) -> Result<Vec<NucleiJsonOutput>> {
    logs::debug!("Scanning with nuclei: {}", domain);
    let mut answers: Vec<NucleiJsonOutput> = vec![];

    let mut target = format!("http://{}", domain);
    if config.scanner.check_force_ssl {
        target = match ssladapter::check_force_https(domain.clone()).await {
            true => format!("https://{}", domain),
            false => format!("http://{}", domain),
        };
    }

    let mut cmd = Command::new(config.scanner.nuclei.nuclei_executable.clone());
    cmd.stdout(std::process::Stdio::piped());
    cmd.args(&config.scanner.nuclei.cli_args);
    cmd.arg("--json").arg("-u").arg(target);

    for template in config.scanner.nuclei.using_community_templates.iter() {
        cmd.arg("-t").arg(template);
    }

    let mut child = cmd.spawn()?;
    let stdout = child.stdout.take().expect("child should have stdout");

    let mut reader = BufReader::new(stdout).lines();

    while let Some(line) = reader.next_line().await? {
        let json: NucleiJsonOutput = match serde_json::from_str(line.trim()) {
            Ok(json) => json,
            Err(e) => {
                logs::debug!("Invalid line received from nuclei stdout reader: {}", e);
                continue;
            }
        };
        
        unsafe {
            if config.database.save_detections {
                // WHY RUST CAN'T JUST HAVE UNSAFE IF
                let db = DATABASE.as_mut().unwrap();
                if db.add_detection(json.clone()) {
                    db.save()?;

                    logs::debug!("New nuclei profits: {:#?}", json);
                    notifications::show_detections_notification(&domain);
                    answers.push(json);
                }
            }
        }
        
    }

    Ok(answers)
}
