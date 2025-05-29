use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerificationEntity {
    pub id: String,
    pub user_id: String,
    pub token: String,
    pub verification_type: VerificationType,
    pub expires_at: DateTime<Utc>,
    pub verified_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VerificationType {
    EmailVerification,
    PasswordReset,
    #[serde(rename = "2fa_setup")]
    TwoFactorSetup,
}

impl From<drivers::database::schemas::verification::Verification> for VerificationEntity {
    fn from(verification: drivers::database::schemas::verification::Verification) -> Self {
        Self {
            id: verification.id,
            user_id: verification.user_id,
            token: verification.token,
            verification_type: match verification.verification_type {
                drivers::database::schemas::verification::VerificationType::EmailVerification => {
                    VerificationType::EmailVerification
                }
                drivers::database::schemas::verification::VerificationType::PasswordReset => {
                    VerificationType::PasswordReset
                }
                drivers::database::schemas::verification::VerificationType::TwoFactorSetup => {
                    VerificationType::TwoFactorSetup
                }
            },
            expires_at: verification.expires_at,
            verified_at: verification.verified_at,
            created_at: verification.created_at,
            updated_at: verification.updated_at,
        }
    }
}

impl From<VerificationEntity> for drivers::database::schemas::verification::Verification {
    fn from(entity: VerificationEntity) -> Self {
        Self {
            id: entity.id,
            user_id: entity.user_id,
            token: entity.token,
            verification_type: match entity.verification_type {
                VerificationType::EmailVerification => {
                    drivers::database::schemas::verification::VerificationType::EmailVerification
                }
                VerificationType::PasswordReset => {
                    drivers::database::schemas::verification::VerificationType::PasswordReset
                }
                VerificationType::TwoFactorSetup => {
                    drivers::database::schemas::verification::VerificationType::TwoFactorSetup
                }
            },
            expires_at: entity.expires_at,
            verified_at: entity.verified_at,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}