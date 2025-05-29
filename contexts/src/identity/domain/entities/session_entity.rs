use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionEntity {
    pub id: String,
    pub user_id: String,
    pub access_token_hash: String,
    pub refresh_token_hash: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub device_fingerprint: Option<String>,
    pub location: Option<String>,
    pub status: SessionStatus,
    pub expires_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionStatus {
    Active,
    Revoked,
    Expired,
}

impl From<drivers::database::schemas::session::Session> for SessionEntity {
    fn from(session: drivers::database::schemas::session::Session) -> Self {
        Self {
            id: session.id,
            user_id: session.user_id,
            access_token_hash: session.access_token_hash,
            refresh_token_hash: session.refresh_token_hash,
            ip_address: session.ip_address,
            user_agent: session.user_agent,
            device_fingerprint: session.device_fingerprint,
            location: session.location,
            status: match session.status {
                drivers::database::schemas::session::SessionStatus::Active => SessionStatus::Active,
                drivers::database::schemas::session::SessionStatus::Revoked => SessionStatus::Revoked,
                drivers::database::schemas::session::SessionStatus::Expired => SessionStatus::Expired,
            },
            expires_at: session.expires_at,
            revoked_at: session.revoked_at,
            created_at: session.created_at,
            updated_at: session.updated_at,
        }
    }
}

impl From<SessionEntity> for drivers::database::schemas::session::Session {
    fn from(entity: SessionEntity) -> Self {
        Self {
            id: entity.id,
            user_id: entity.user_id,
            access_token_hash: entity.access_token_hash,
            refresh_token_hash: entity.refresh_token_hash,
            ip_address: entity.ip_address,
            user_agent: entity.user_agent,
            device_fingerprint: entity.device_fingerprint,
            location: entity.location,
            status: match entity.status {
                SessionStatus::Active => drivers::database::schemas::session::SessionStatus::Active,
                SessionStatus::Revoked => drivers::database::schemas::session::SessionStatus::Revoked,
                SessionStatus::Expired => drivers::database::schemas::session::SessionStatus::Expired,
            },
            expires_at: entity.expires_at,
            revoked_at: entity.revoked_at,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}