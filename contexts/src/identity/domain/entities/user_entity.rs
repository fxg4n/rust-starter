use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserEntity {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserStatus {
    Pending,
    Active,
    Banned,
    Deactivated,
}

impl From<drivers::database::schemas::user::User> for UserEntity {
    fn from(user: drivers::database::schemas::user::User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            password_hash: user.password_hash,
            password_algorithm: user.password_algorithm,
            password_version: user.password_version,
            password_last_changed: user.password_last_changed,
            status: match user.status {
                drivers::database::schemas::user::UserStatus::Pending => UserStatus::Pending,
                drivers::database::schemas::user::UserStatus::Active => UserStatus::Active,
                drivers::database::schemas::user::UserStatus::Banned => UserStatus::Banned,
                drivers::database::schemas::user::UserStatus::Deactivated => UserStatus::Deactivated,
            },
            last_login: user.last_login,
            activated_at: user.activated_at,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

impl From<UserEntity> for drivers::database::schemas::user::User {
    fn from(entity: UserEntity) -> Self {
        Self {
            id: entity.id,
            username: entity.username,
            email: entity.email,
            password_hash: entity.password_hash,
            password_algorithm: entity.password_algorithm,
            password_version: entity.password_version,
            password_last_changed: entity.password_last_changed,
            status: match entity.status {
                UserStatus::Pending => drivers::database::schemas::user::UserStatus::Pending,
                UserStatus::Active => drivers::database::schemas::user::UserStatus::Active,
                UserStatus::Banned => drivers::database::schemas::user::UserStatus::Banned,
                UserStatus::Deactivated => drivers::database::schemas::user::UserStatus::Deactivated,
            },
            last_login: entity.last_login,
            activated_at: entity.activated_at,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}