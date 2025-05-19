use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub hash_rt: Option<String>,
    pub is_active: bool,
    pub last_login: Option<DateTime<Utc>>,
    pub activation_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<crate::drivers::database::schemas::user::User> for User {
    fn from(model: crate::drivers::database::schemas::user::User) -> Self {
        Self {
            id: model.id,
            username: model.username,
            email: model.email,
            password_hash: model.password_hash,
            hash_rt: model.hash_rt,
            is_active: model.is_active,
            last_login: model.last_login,
            activation_at: model.activation_at,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}