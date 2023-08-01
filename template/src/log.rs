use crate::config::GlobalConfig;
use std::str::FromStr;
use tracing::warn;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::{fmt, prelude::*};

pub async fn configure_log(global_config: &GlobalConfig) {
    let subscriber =
        tracing_subscriber::registry().with(fmt::layer().with_file(true).with_line_number(true));

    let mut is_fall_back = false;

    if global_config.log.file.enabled {
        let file_writer =
            tracing_appender::rolling::daily(global_config.log.file.path.to_string(), "cuba.log");

        subscriber
            .with(
                fmt::layer()
                    .with_file(true)
                    .with_line_number(true)
                    .with_ansi(false)
                    .with_writer(file_writer),
            )
            .with(
                LevelFilter::from_str(global_config.log.level.as_ref()).unwrap_or_else(|_| {
                    is_fall_back = true;
                    LevelFilter::INFO
                }),
            )
            .init();
    } else {
        subscriber
            .with(
                LevelFilter::from_str(global_config.log.level.as_ref()).unwrap_or_else(|_| {
                    is_fall_back = true;
                    LevelFilter::INFO
                }),
            )
            .init();
    }
    if is_fall_back {
        warn!("invalid log level specified, fall back to info level")
    }
}
