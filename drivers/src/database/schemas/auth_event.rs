use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AuthEvent {
    pub id: String,
    pub user_id: Option<String>,
    pub event_type: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub context: Option<String>,
    pub success: Option<bool>,
    pub created_at: DateTime<Utc>,
}