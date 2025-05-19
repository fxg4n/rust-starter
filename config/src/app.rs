use super::env;
use anyhow::Result;
use core::constants::logging::DEFAULT_LOG_LEVEL;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub environment: String,
    pub log_level: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            host: env::get_var("APP_HOST")?,
            port: env::get_var("APP_PORT")?,
            environment: env::get_var_or("APP_ENV", "development".to_string()),
            log_level: env::get_var_or("APP_LOG_LEVEL", DEFAULT_LOG_LEVEL.to_string()),
        })
    }
}
