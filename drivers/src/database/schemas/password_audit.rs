use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PasswordAudit {
    pub id: String,
    pub user_id: String,
    pub password_hash: String,
    pub password_algorithm: String,
    pub context: Option<String>,
    pub encryption_key_id: Option<String>,
    pub encrypted_backup: Option<Vec<u8>>,
    pub encryption_iv: Option<Vec<u8>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}