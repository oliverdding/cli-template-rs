use crate::config::GlobalConfig;
use std::str::FromStr;
use tracing::warn;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::{fmt, prelude::*};

pub async fn configure_log(global_config: &GlobalConfig) {
    let mut is_fall_back = false;

    let subscriber = tracing_subscriber::registry()
        .with(
            LevelFilter::from_str(&global_config.log.level).unwrap_or_else(|_| {
                is_fall_back = true;
                LevelFilter::INFO
            }),
        )
        .with(fmt::layer().with_file(true).with_line_number(true));

    if global_config.log.file.enabled {
        let file_writer = tracing_appender::rolling::daily(
            global_config.log.file.path.to_string(),
            "rolling.log",
        );

        subscriber
            .with(
                fmt::layer()
                    .with_file(true)
                    .with_line_number(true)
                    .with_ansi(false)
                    .with_writer(file_writer),
            )
            .init();
    } else {
        subscriber.init();
    }

    if is_fall_back {
        warn!(
            "invalid log level '{}', fall back to info level",
            &global_config.log.level
        )
    }
}
