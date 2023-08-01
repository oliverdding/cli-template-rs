mod cli;
mod commands;
mod config;
mod log;

use clap::Parser;
use cli::{Cli, Commands};
use commands::completion;
use miette::Result;
use tokio::time::Duration;
use tokio_graceful_shutdown::Toplevel;
use tracing::trace;
use tracing::debug;
use tracing::error;
use tracing::info;
use tracing::warn;

use crate::commands::command1;
use crate::commands::command2;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let global_config = config::GlobalConfig::new(&cli).await?;
    log::configure_log(&global_config).await;

    match cli.command {
        Commands::Command1 => Toplevel::new()
            .start("command1", command1::run)
            .catch_signals()
            .handle_shutdown_requests(Duration::from_millis(1000))
            .await
            .map_err(Into::into),
        Commands::Command2 => Toplevel::new()
            .start("command2", command2::run)
            .catch_signals()
            .handle_shutdown_requests(Duration::from_millis(1000))
            .await
            .map_err(Into::into),
        Commands::Completion { shell } => {
            completion::run(shell).await;
            Ok(())
        }
    }
}
