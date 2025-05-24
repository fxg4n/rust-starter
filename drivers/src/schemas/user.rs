use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub password_algorithm: String,
    pub password_version: i32,
    pub password_last_changed: Option<DateTime<Utc>>,
    pub status: UserStatus,
    pub last_login: Option<DateTime<Utc>>,
    pub activated_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum UserStatus {
    Pending,
    Active,
    Banned,
    Deactivated,
}