use super::env;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct ServicesConfig {
    pub aws: AwsConfig,
    pub smtp: SmtpConfig,
}

#[derive(Debug, Clone)]
pub struct AwsConfig {
    pub access_key_id: String,
    pub secret_access_key: String,
    pub region: String,
    pub s3_bucket: String,
}

#[derive(Debug, Clone)]
pub struct SmtpConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub from_email: String,
    pub from_name: String,
}

impl ServicesConfig {
    pub fn from_env() -> Result<Self>  {
        Ok(Self {
            aws: AwsConfig::from_env()?,
            smtp: SmtpConfig::from_env()?,
        })
    }
}

impl AwsConfig {
    pub fn from_env() -> Result<Self>  {
        Ok(Self {
            access_key_id: env::get_var("AWS_ACCESS_KEY_ID")?,
            secret_access_key: env::get_var("AWS_SECRET_ACCESS_KEY")?,
            region: env::get_var_or("AWS_REGION", "us-east-1".to_string()),
            s3_bucket: env::get_var("AWS_S3_BUCKET")?,
        })
    }
}

impl SmtpConfig {
    pub fn from_env() -> Result<Self>  {
        Ok(Self {
            host: env::get_var_or("SMTP_HOST", "smtp.gmail.com".to_string()),
            port: env::get_var_or("SMTP_PORT", 587),
            username: env::get_var("SMTP_USERNAME")?,
            password: env::get_var("SMTP_PASSWORD")?,
            from_email: env::get_var("SMTP_FROM_EMAIL")?,
            from_name: env::get_var("SMTP_FROM_NAME")?,
        })
    }
}