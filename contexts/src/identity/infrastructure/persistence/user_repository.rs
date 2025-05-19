use anyhow::Result;
use sqlx::PgPool;
use crate::contexts::identity::domain::entities::User;
use crate::infrastructure::database::models;

pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, models::User>(
            "SELECT * FROM users WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user.map(User::from))
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, models::User>(
            "SELECT * FROM users WHERE email = $1"
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user.map(User::from))
    }

    pub async fn create(&self, user: &models::User) -> Result<User> {
        let created = sqlx::query_as::<_, models::User>(
            r#"
            INSERT INTO users (id, username, email, password_hash, is_active)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#
        )
        .bind(&user.id)
        .bind(&user.username)
        .bind(&user.email)
        .bind(&user.password_hash)
        .bind(user.is_active)
        .fetch_one(&self.pool)
        .await?;

        Ok(User::from(created))
    }

    pub async fn update(&self, user: &User) -> Result<User> {
        let updated = sqlx::query_as::<_, models::User>(
            r#"
            UPDATE users
            SET username = $2, email = $3, is_active = $4,
                last_login = $5, activation_at = $6, hash_rt = $7
            WHERE id = $1
            RETURNING *
            "#
        )
        .bind(&user.id)
        .bind(&user.username)
        .bind(&user.email)
        .bind(user.is_active)
        .bind(user.last_login)
        .bind(user.activation_at)
        .bind(&user.hash_rt)
        .fetch_one(&self.pool)
        .await?;

        Ok(User::from(updated))
    }

    pub async fn delete(&self, id: &str) -> Result<()> {
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}