use anyhow::Result;
use self::scanners::Scanner;

pub mod scanners;

#[derive(Debug)]
pub struct ScanAnswer {
    pub full_url: String,
    pub detection_name: String
}

pub struct ServicesScanner {
    scanners: Vec<Box<dyn Scanner + Send + Sync>>
}

impl ServicesScanner {
    pub fn new(scanners: Vec<Box<dyn Scanner + Send + Sync>>) -> Self {
        Self { scanners }
    }

    pub async fn scan(&self, target: String) -> Result<Vec<ScanAnswer>> {
        logs::debug!("Scanning: {}", target);
        let mut answers: Vec<ScanAnswer> = vec![];

        for scanner in self.scanners.iter() {
            logs::info!("Performing {} on {}", scanner.name(), target);
            answers.append(&mut scanner.process(&target).await?);
        }

        // println!("{:#?}", answers);
        Ok(answers)
    }
}