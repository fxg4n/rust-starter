use std::str::FromStr;
use tracing::Level;
use tracing_subscriber::{
    fmt::{self, format::FmtSpan, time::SystemTime},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter, Layer,
};
use crate::core::constants::logging::*;

pub struct LogConfig {
    pub app_name: String,
    pub log_level: String,
    pub json_output: bool,
    pub file_output: Option<String>,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            app_name: "app".to_string(),
            log_level: DEFAULT_LOG_LEVEL.to_string(),
            json_output: DEFAULT_JSON_OUTPUT,
            file_output: None,
        }
    }
}

pub fn init_with_config(config: LogConfig) {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        let mut filter_str = format!("{}={}", config.app_name, config.log_level);
        for component in DEFAULT_COMPONENTS {
            filter_str.push_str(&format!(",{}", component));
        }
        EnvFilter::new(filter_str)
    });

    let fmt_layer = fmt::layer()
        .with_target(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_file(true)
        .with_line_number(true)
        .with_timer(SystemTime)
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE);

    let registry = tracing_subscriber::registry()
        .with(env_filter)
        .with(if config.json_output {
            fmt_layer.json().boxed()
        } else {
            fmt_layer.boxed()
        });

    if let Some(file_path) = config.file_output {
        if let Ok(file) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_path)
        {
            let file_layer = fmt::layer()
                .with_target(true)
                .with_thread_ids(true)
                .with_file(true)
                .with_line_number(true)
                .with_writer(file)
                .json();
            registry.with(file_layer).init();
        } else {
            registry.init();
        }
    } else {
        registry.init();
    }

    tracing::info!(
        app_name = config.app_name,
        log_level = config.log_level,
        "Logging initialized"
    );
}

pub fn init(app_name: &str, log_level: &str) {
    init_with_config(LogConfig {
        app_name: app_name.to_string(),
        log_level: log_level.to_string(),
        json_output: true,
        file_output: None,
    });
}

pub fn parse_log_level(level: &str) -> Level {
    Level::from_str(level).unwrap_or(Level::INFO)
}
