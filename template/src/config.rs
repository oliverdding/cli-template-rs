use config::{Config, Environment, File};
use miette::{IntoDiagnostic, Result};
use serde::{Deserialize, Serialize};
use std::{env, path::PathBuf};

use crate::cli;

// default value <- global configuration file <- user configuration file <- environmen variables <- command line arguments
#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalConfig {
    pub log: Log,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Log {
    pub level: String,
    pub file: LogFile,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogFile {
    pub enabled: bool,
    pub path: String,
}

impl GlobalConfig {
    pub async fn new(cli: &cli::Cli) -> Result<GlobalConfig> {
        let xdg_base = match xdg::BaseDirectories::with_prefix("template-project") {
            Ok(v) => v,
            Err(e) => panic!("error getting xdg directories: {}", e), // FIXME: return error instead of panic
        };

        let home_path: PathBuf =
            cli.home
                .clone()
                .unwrap_or_else(|| match env::var("TEMPLATE_PROJECT_HOME") {
                    Ok(v) => PathBuf::from(v),
                    Err(_) => xdg_base.get_config_home(),
                });

        let mut config_builder = Config::builder()
            .set_default("log.level", "info")
            .into_diagnostic()?
            .set_default("log.file.enabled", false)
            .into_diagnostic()?
            .set_default("log.file.path", xdg_base.get_cache_home().to_str())
            .into_diagnostic()?
            .add_source(File::with_name("/etc/cuba/config").required(false))
            .add_source(File::from(home_path.join("config")).required(false))
            .add_source(
                Environment::with_prefix("CUBA")
                    .separator("_")
                    .ignore_empty(true),
            );

        config_builder = if let Some(log_level) = cli.log_level {
            config_builder
                .set_override("log.level", log_level.as_str())
                .unwrap()
        } else {
            config_builder
        };

        config_builder = if let Some(log_file_enable) = cli.log_file_enable {
            config_builder
                .set_override("log.file.enabled", log_file_enable)
                .unwrap()
        } else {
            config_builder
        };

        config_builder = if let Some(log_file_path) = cli.log_file_path.clone() {
            config_builder
                .set_override("log.file.path", log_file_path.to_str())
                .unwrap()
        } else {
            config_builder
        };

        Ok(config_builder
            .build()
            .into_diagnostic()?
            .try_deserialize()
            .into_diagnostic()?)
    }
}
