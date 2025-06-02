use super::env;
use anyhow::Result;
use core::constants::logging::DEFAULT_LOG_LEVEL;
use core::constants::app::{DEFAULT_ENDPOINT_PREFIX, DEFAULT_ENVIRONTMENT, DEFAULT_TIMEZONE};
use shared::utils::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub environment: String,
    pub log_level: String,
    pub endpoint_prefix: String,
    pub cors_origins: Vec<String>,
    pub timezone: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            host: env::get_var("APP_HOST")?,
            port: env::get_var("APP_PORT")?,
            environment: env::get_var_or("APP_ENV", DEFAULT_ENVIRONTMENT.to_string()),
            log_level: env::get_var_or("APP_LOG_LEVEL", DEFAULT_LOG_LEVEL.to_string()),
            endpoint_prefix: env::get_var_or("APP_ENDPOINT_PREFIX", DEFAULT_ENDPOINT_PREFIX.to_string()),
            cors_origins: env::get_var_or("APP_CORS_ORIGINS", "*".to_string())
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
            timezone: env::get_var_or("APP_TIMEZONE", DEFAULT_TIMEZONE.to_string()),
        })
    }
}
