use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionResponse {
    pub id: String,
    pub user_id: String,
    pub ip_address: String,
    pub user_agent: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSessionRequest {
    pub user_id: String,
    pub ip_address: String,
    pub user_agent: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSessionRequest {
    pub is_active: bool,
}

impl From<crate::contexts::identity::domain::entities::session::Session> for SessionResponse {
    fn from(session: crate::contexts::identity::domain::entities::session::Session) -> Self {
        Self {
            id: session.id,
            user_id: session.user_id,
            ip_address: session.ip_address,
            user_agent: session.user_agent,
            is_active: session.is_active,
            created_at: session.created_at,
            updated_at: session.updated_at,
        }
    }
}