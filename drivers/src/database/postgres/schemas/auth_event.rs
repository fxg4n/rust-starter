use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthEvent {
    pub id: String,
    pub user_id: Option<String>,
    pub event_type: AuthEventType,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub context: Option<String>,
    pub success: Option<bool>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum AuthEventType {
    Login,
    Logout,
    #[serde(rename = "2fa_attempt")]
    TwoFactorAttempt,
    PasswordReset,
}