use clap::{Parser, Subcommand};
use logs::LevelFilter;
use anyhow::Result;

mod cmds;

#[derive(Parser)]
#[command()]
struct CliArgs {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Ping,
}

#[tokio::main]
async fn main() -> Result<()> {
    let logger = logs::Logs::new().color(true);
    match logger.level_from_env("HOGG_CLI_LOG") {
        Ok(logger) => {
            logger.init();
        }
        Err(_) => {
            logs::Logs::new().color(true).level(LevelFilter::Info).init();
        }
    };

    match CliArgs::parse().command {
        Some(Commands::Ping) => cmds::ping_command().await?,
        None => logs::error!("No command given"),
    }

    Ok(())
}
