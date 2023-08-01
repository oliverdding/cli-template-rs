use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};
use clap_complete::Shell;

#[derive(Parser)]
#[command(
    author,
    version,
    about,
    long_about = None
)]
pub struct Cli {
    #[arg(short = 'H', long, global = true, value_name = "PATH")]
    pub home: Option<PathBuf>,

    #[arg(short = 'l', long, global = true, value_name = "LEVEL", value_enum)]
    pub log_level: Option<LogLevel>,

    #[arg(long, global = true, value_name = "BOOL")]
    pub log_file_enable: Option<bool>,

    #[arg(long, global = true, value_name = "PATH")]
    pub log_file_path: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl LogLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Trace => "trace",
            LogLevel::Debug => "debug",
            LogLevel::Info => "info",
            LogLevel::Warn => "warn",
            LogLevel::Error => "error",
        }
    }
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Run command1")]
    Command1,
    #[command(about = "Run command2")]
    Command2,
    #[command(about = "Generate shell completion script", disable_help_flag = true)]
    Completion {
        #[arg(
            long,
            value_name = "SHELL",
            help = "The shell to generate the completions for",
            value_enum
        )]
        shell: Shell,
    },
}
