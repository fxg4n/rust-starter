use tracing::Level;
use std::time::Duration;

pub const DEFAULT_LOG_LEVEL: Level = Level::INFO;
pub const DEFAULT_JSON_OUTPUT: bool = true;
pub const LOG_FLUSH_INTERVAL: Duration = Duration::from_secs(1);
pub const MAX_LOG_FILE_SIZE: usize = 100 * 1024 * 1024;
pub const MAX_LOG_FILES: usize = 5;
pub const LOG_FILE_PREFIX: &str = "app";
pub const LOG_DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S%.3f";
pub const DEFAULT_COMPONENTS: &[&str] = &[
    "tower_http=debug",
    "axum::rejection=trace",
    "sqlx=warn",
];
pub const REQUEST_ID_FIELD: &str = "request.id";
pub const TIMESTAMP_FIELD: &str = "timestamp";
pub const ERROR_TYPE_FIELD: &str = "error.type_name";
pub const ERROR_DISPLAY_FIELD: &str = "error.display";
pub const ERROR_DEBUG_FIELD: &str = "error.debug";