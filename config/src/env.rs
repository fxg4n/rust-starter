use std::env;
use std::time::Duration;
use anyhow::{Result, Context, anyhow};
use dotenv::dotenv;

pub fn init() {
    dotenv().ok();
}

pub fn get_var<T: std::str::FromStr>(key: &str) -> Result<T> {
    let value = env::var(key)
        .with_context(|| format!("Failed to get environment variable: {}", key))?;
    
    value.parse::<T>()
        .map_err(|_| anyhow!("Failed to parse environment variable: {}", key))
}

pub fn get_var_or<T: std::str::FromStr>(key: &str, default: T) -> T {
    env::var(key)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

pub fn get_duration_or(key: &str, default: Duration) -> Duration {
    env::var(key)
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .map(Duration::from_secs)
        .unwrap_or(default)
}

pub fn get_env_or_default(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}