use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswordAuditEntity {
    pub id: String,
    pub user_id: String,
    pub password_hash: String,
    pub password_algorithm: String,
    pub context: Option<String>,
    pub encryption_key_id: Option<String>,
    pub encrypted_backup: Option<Vec<u8>>,
    pub encryption_iv: Option<Vec<u8>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl From<drivers::database::schemas::password_audit::PasswordAudit> for PasswordAuditEntity {
    fn from(audit: drivers::database::schemas::password_audit::PasswordAudit) -> Self {
        Self {
            id: audit.id,
            user_id: audit.user_id,
            password_hash: audit.password_hash,
            password_algorithm: audit.password_algorithm,
            context: audit.context,
            encryption_key_id: audit.encryption_key_id,
            encrypted_backup: audit.encrypted_backup,
            encryption_iv: audit.encryption_iv,
            expires_at: audit.expires_at,
            revoked_at: audit.revoked_at,
            created_at: audit.created_at,
        }
    }
}

impl From<PasswordAuditEntity> for drivers::database::schemas::password_audit::PasswordAudit {
    fn from(entity: PasswordAuditEntity) -> Self {
        Self {
            id: entity.id,
            user_id: entity.user_id,
            password_hash: entity.password_hash,
            password_algorithm: entity.password_algorithm,
            context: entity.context,
            encryption_key_id: entity.encryption_key_id,
            encrypted_backup: entity.encrypted_backup,
            encryption_iv: entity.encryption_iv,
            expires_at: entity.expires_at,
            revoked_at: entity.revoked_at,
            created_at: entity.created_at,
        }
    }
}