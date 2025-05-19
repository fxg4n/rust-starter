use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Session {
    pub id: String,
    pub user_id: String,
    pub ip_address: String,
    pub user_agent: String,
    pub hash_at: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<crate::drivers::database::schemas::session::Session> for Session {
    fn from(model: crate::drivers::database::schemas::session::Session) -> Self {
        Self {
            id: model.id,
            user_id: model.user_id,
            ip_address: model.ip_address,
            user_agent: model.user_agent,
            hash_at: model.hash_at,
            is_active: model.is_active,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}