use super::env;
use core::constants::database::*;
use std::time::Duration;
use anyhow::Result;


#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database_name: String,
    pub ssl_mode: String,
    pub statement_timeout: Duration,
    pub pool_timeout: Duration,
    pub max_connections: u32,
}

impl DatabaseConfig {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            host: env::get_var("DB_HOST")?,
            port: env::get_var("DB_PORT")?,
            username: env::get_var("DB_USER")?,
            password: env::get_var("DB_PASSWORD")?,
            database_name: env::get_var("DB_NAME")?,
            ssl_mode: env::get_var_or("DB_SSL_MODE", "prefer".to_string()),
            statement_timeout: env::get_duration_or("DB_STATEMENT_TIMEOUT", STATEMENT_TIMEOUT),
            pool_timeout: env::get_duration_or("DB_POOL_TIMEOUT", POOL_TIMEOUT),
            max_connections: env::get_var_or("DB_MAX_CONNECTIONS", MAX_CONNECTIONS),
        })
    }

    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}?sslmode={}&statement_timeout={}",
            self.username,
            self.password,
            self.host,
            self.port,
            self.database_name,
            self.ssl_mode,
            self.statement_timeout.as_millis()
        )
    }
}