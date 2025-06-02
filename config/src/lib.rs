pub mod app;
pub mod database;
pub mod services;
pub mod redis;
pub mod jwt;

use anyhow::Result;
use app::AppConfig;
use database::DatabaseConfig;
use services::ServicesConfig;
use redis::RedisConfig;
use jwt::JwtConfig;

#[derive(Debug, Clone)]
pub struct Config {
    pub app: AppConfig,
    pub database: DatabaseConfig,
    pub services: ServicesConfig,
    pub redis: RedisConfig,
    pub jwt: JwtConfig,
}

impl Config {
    pub fn load() -> Result<Self> {
        Ok(Self {
            app: AppConfig::from_env()?,
            database: DatabaseConfig::from_env()?,
            services: ServicesConfig::from_env()?,
            redis: RedisConfig::from_env()?,
            jwt: JwtConfig::from_env()?,
        })
    }
}
