use anyhow::Result;
use clap::{Parser, Subcommand};
use logs::LevelFilter;

mod cmds;

#[derive(Parser)]
#[command(name = "hogg-cli", version = "0.1.0", author = "yallxe")]
#[command(about = "A CLI for Hogg")]
struct CliArgs {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Ping the Hogg Daemon to check if it's running")]
    Ping,
    #[command(about = "Get all unviewed detections from hogg database")]
    UnviewedDetections,
    #[command(about = "Delete all the detections from hogg database")]
    Flush
}

#[tokio::main]
async fn main() -> Result<()> {
    let logger = logs::Logs::new().color(true);
    match logger.level_from_env("HOGG_CLI_LOG") {
        Ok(logger) => {
            logger.init();
        }
        Err(_) => {
            logs::Logs::new()
                .color(true)
                .level(LevelFilter::Info)
                .init();
        }
    };

    match CliArgs::parse().command {
        Some(Commands::Ping) => cmds::ping_command().await?,
        Some(Commands::UnviewedDetections) => cmds::unviewed_detections_command().await?,
        Some(Commands::Flush) => cmds::flush_command().await?,
        None => logs::error!("No command given"),
    }

    Ok(())
}
