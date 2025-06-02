use super::env;
use anyhow::Result;
use shared::utils::env;

#[derive(Debug, Clone)]
pub struct VaultConfig {
    pub address: String,
    pub token: String,
    pub mount_path: String,
    pub namespace: Option<String>,
    pub engine: String,
}

impl VaultConfig {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            address: env::get_var("VAULT_ADDR")?,
            token: env::get_var("VAULT_TOKEN")?,
            mount_path: env::get_var_or("VAULT_MOUNT_PATH", "secret".to_string())?,
            namespace: env::get_optional("VAULT_NAMESPACE")?,
            engine: env::get_var_or("VAULT_ENGINE", "kv".to_string())?,
        })
    }
}
