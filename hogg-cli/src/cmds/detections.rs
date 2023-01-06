use std::path::Path;

use anyhow::Result;
use hogg_common::db::HoggDatabase;
use hogg_common::env;
use hogg_daemon::nuclei::NucleiJsonOutput;
use hogg_grpc::grpc;

fn get_database_dir() -> String {
    Path::new(&env::get_hogg_dir())
        .join(".hoggdb.json")
        .to_str()
        .unwrap()
        .to_string()
}

pub async fn get_unviewed() -> Result<()> {
    let mut db = HoggDatabase::<NucleiJsonOutput>::from_file_unconfigured(get_database_dir())?;

    let detections = db.get_unviewed_detections(true)?;

    println!();
    for d in detections.iter() {
        println!(
            "Vulnerability {} - {}",
            d.data.info.name, d.data.info.severity
        );
        println!(
            " - Host: {}", 
            d.data.host
        );
        println!(
            " - Matched at: {}",
            d.data.matched_at.clone().unwrap_or("".to_string())
        );
        println!(
            " - Description: {}",
            d.data
                .info
                .description
                .clone()
                .unwrap_or("No description".to_string())
                .trim()
        );
        println!(
            " - References: {}",
            d.data
                .info
                .reference
                .clone()
                .unwrap_or(vec!["None".to_string()])
                .join(", ")
        );
        
        println!(" - Timestamp: {}", d.data.timestamp);
        println!();
        println!();
    }
    logs::info!("There were {} unviewed detections", detections.len());
    Ok(())
}

pub async fn flush_detections() -> Result<()> {
    logs::info!("Flushing detections from database...");
    let mut db = HoggDatabase::<NucleiJsonOutput>::from_file_unconfigured(get_database_dir())?;
    db.flush_detections()?;

    logs::info!("Sending database forced reload to hogg-daemon...");
    let mut grpc = grpc::connect_grpc_client().await?;
    grpc.reload_database(grpc::ReloadDatabaseRequest {}).await?;

    Ok(())
}
