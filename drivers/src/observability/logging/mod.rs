pub mod init;
pub mod macros;
pub mod request_id;

pub use init::{init, parse_log_level, LogConfig};
pub use request_id::RequestId;

pub use crate::core::constants::logging::{
    DEFAULT_LOG_LEVEL,
    DEFAULT_JSON_OUTPUT,
    LOG_FLUSH_INTERVAL,
};
