use std::path::Path;

use anyhow::Result;

#[derive(Debug)]
pub struct ScanAnswer {
    pub full_url: String,
    pub detection_name: String,
}

pub struct ServicesScanner {
    nuclei_path: String,
}

impl ServicesScanner {
    pub fn new(nuclei_path: String) -> Self {
        Self { nuclei_path }
    }

    pub async fn scan(&self, target: String) -> Result<Vec<ScanAnswer>> {
        logs::debug!("Scanning: {}", target);
        let mut answers: Vec<ScanAnswer> = vec![];
        // TODO: Nuclei scan

        Ok(answers)
    }
}
