use anyhow::Result;
use sqlx::PgPool;
use crate::contexts::identity::domain::entities::Session;
use crate::infrastructure::database::models;

pub struct SessionRepository {
    pool: PgPool,
}

impl SessionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<Session>> {
        let session = sqlx::query_as::<_, models::Session>(
            "SELECT * FROM sessions WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(session.map(Session::from))
    }

    pub async fn find_active_by_user_id(&self, user_id: &str) -> Result<Vec<Session>> {
        let sessions = sqlx::query_as::<_, models::Session>(
            "SELECT * FROM sessions WHERE user_id = $1 AND is_active = true"
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(sessions.into_iter().map(Session::from).collect())
    }

    pub async fn create(&self, session: &models::Session) -> Result<Session> {
        let created = sqlx::query_as::<_, models::Session>(
            r#"
            INSERT INTO sessions (id, user_id, ip_address, user_agent, hash_at, is_active)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#
        )
        .bind(&session.id)
        .bind(&session.user_id)
        .bind(&session.ip_address)
        .bind(&session.user_agent)
        .bind(&session.hash_at)
        .bind(session.is_active)
        .fetch_one(&self.pool)
        .await?;

        Ok(Session::from(created))
    }

    pub async fn update_status(&self, id: &str, is_active: bool) -> Result<Session> {
        let updated = sqlx::query_as::<_, models::Session>(
            "UPDATE sessions SET is_active = $2 WHERE id = $1 RETURNING *"
        )
        .bind(id)
        .bind(is_active)
        .fetch_one(&self.pool)
        .await?;

        Ok(Session::from(updated))
    }

    pub async fn deactivate_all_user_sessions(&self, user_id: &str) -> Result<()> {
        sqlx::query(
            "UPDATE sessions SET is_active = false WHERE user_id = $1"
        )
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete_expired(&self) -> Result<u64> {
        let result = sqlx::query(
            "DELETE FROM sessions WHERE is_active = false"
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }
}