use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VerificationResponse {
    pub id: String,
    pub user_id: String,
    pub expired_at: DateTime<Utc>,
    pub verified: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateVerificationRequest {
    pub user_id: String,
    pub expired_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyTokenRequest {
    pub token: String,
}

impl From<crate::contexts::identity::domain::entities::verification::Verification> for VerificationResponse {
    fn from(verification: crate::contexts::identity::domain::entities::verification::Verification) -> Self {
        Self {
            id: verification.id,
            user_id: verification.user_id,
            expired_at: verification.expired_at,
            verified: verification.verified,
            created_at: verification.created_at,
        }
    }
}