use anyhow::Result;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use crate::contexts::identity::domain::entities::Verification;
use crate::infrastructure::database::models;

pub struct VerificationRepository {
    pool: PgPool,
}

impl VerificationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_token(&self, token: &str) -> Result<Option<Verification>> {
        let verification = sqlx::query_as::<_, models::Verification>(
            "SELECT * FROM verifications WHERE token = $1"
        )
        .bind(token)
        .fetch_optional(&self.pool)
        .await?;

        Ok(verification.map(Verification::from))
    }

    pub async fn find_active_by_user_id(&self, user_id: &str) -> Result<Option<Verification>> {
        let verification = sqlx::query_as::<_, models::Verification>(
            r#"
            SELECT * FROM verifications 
            WHERE user_id = $1 
            AND verified = false 
            AND expired_at > CURRENT_TIMESTAMP
            "#
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(verification.map(Verification::from))
    }

    pub async fn create(&self, verification: &models::Verification) -> Result<Verification> {
        let created = sqlx::query_as::<_, models::Verification>(
            r#"
            INSERT INTO verifications (id, user_id, token, expired_at, verified)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#
        )
        .bind(&verification.id)
        .bind(&verification.user_id)
        .bind(&verification.token)
        .bind(verification.expired_at)
        .bind(verification.verified)
        .fetch_one(&self.pool)
        .await?;

        Ok(Verification::from(created))
    }

    pub async fn mark_as_verified(&self, token: &str) -> Result<Verification> {
        let updated = sqlx::query_as::<_, models::Verification>(
            r#"
            UPDATE verifications 
            SET verified = true 
            WHERE token = $1 
            AND verified = false 
            AND expired_at > CURRENT_TIMESTAMP
            RETURNING *
            "#
        )
        .bind(token)
        .fetch_one(&self.pool)
        .await?;

        Ok(Verification::from(updated))
    }

    pub async fn delete_expired(&self) -> Result<u64> {
        let result = sqlx::query(
            "DELETE FROM verifications WHERE expired_at <= CURRENT_TIMESTAMP"
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }

    pub async fn invalidate_user_tokens(&self, user_id: &str) -> Result<u64> {
        let result = sqlx::query(
            "UPDATE verifications SET expired_at = CURRENT_TIMESTAMP WHERE user_id = $1"
        )
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }
}