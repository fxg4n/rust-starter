use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Verification {
    pub id: String,
    pub user_id: String,
    pub token: String,
    pub expired_at: DateTime<Utc>,
    pub verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<crate::drivers::database::schemas::verification::Verification> for Verification {
    fn from(model: crate::drivers::database::schemas::verification::Verification) -> Self {
        Self {
            id: model.id,
            user_id: model.user_id,
            token: model.token,
            expired_at: model.expired_at,
            verified: model.verified,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}