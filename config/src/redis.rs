use super::env;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct RedisConfig {
    pub host: String,
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
    pub database: u8,
}

impl RedisConfig {
    pub fn from_env() -> Result<Self>  {
        Ok(Self {
            host: env::get_var("REDIS_HOST")?,
            port: env::get_var("REDIS_PORT")?,
            username: env::get_var("REDIS_USERNAME").ok(),
            password: env::get_var("REDIS_PASSWORD").ok(),
            database: env::get_var_or("REDIS_DATABASE", 0),
        })
    }

    pub fn connection_string(&self) -> String {
        let auth = match (&self.username, &self.password) {
            (Some(user), Some(pass)) => format!("{}:{}@", user, pass),
            (None, Some(pass)) => format!(":{}", pass),
            _ => String::new(),
        };

        format!("redis://{}{}:{}/{}", auth, self.host, self.port, self.database)
    }
}