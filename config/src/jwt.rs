use super::env;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct JwtConfig {
    pub access_token_secret: String,
    pub refresh_token_secret: String,
}

impl JwtConfig {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            access_token_secret: env::get_var("JWT_ACCESS_TOKEN_SECRET")?,
            refresh_token_secret: env::get_var("JWT_REFRESH_TOKEN_SECRET")?,
        })
    }
}