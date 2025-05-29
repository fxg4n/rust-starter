use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::identity::domain::entities::user_entity::{UserEntity, UserStatus};
use anyhow::Result;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: &UserEntity) -> Result<UserEntity>;
    async fn update(&self, user: &UserEntity) -> Result<UserEntity>;
    async fn delete(&self, id: &str) -> Result<()>;
    async fn find_by_id(&self, id: &str) -> Result<Option<UserEntity>>;
    async fn find_by_email(&self, email: &str) -> Result<Option<UserEntity>>;
    async fn find_by_username(&self, username: &str) -> Result<Option<UserEntity>>;
    async fn find_by_status(&self, status: &UserStatus) -> Result<Vec<UserEntity>>;
    async fn find_by_last_login_before(&self, date: &DateTime<Utc>) -> Result<Vec<UserEntity>>;
    async fn find_inactive_users(&self, days: i64) -> Result<Vec<UserEntity>>;
    async fn update_status(&self, id: &str, status: UserStatus) -> Result<UserEntity>;
    async fn update_password(
        &self,
        id: &str,
        password_hash: &str,
        algorithm: &str,
        version: i32,
    ) -> Result<UserEntity>;
    async fn update_last_login(&self, id: &str) -> Result<UserEntity>;
    async fn exists_by_email(&self, email: &str) -> Result<bool>;
    async fn exists_by_username(&self, username: &str) -> Result<bool>;
}

pub struct PostgresUserRepository {
    pool: deadpool_postgres::Pool,
}

impl PostgresUserRepository {
    pub fn new(pool: deadpool_postgres::Pool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create(&self, user: &UserEntity) -> Result<UserEntity> {
        let client = self.pool.get().await?;
        let row = client
            .query_one(
                "INSERT INTO users (
                    id, username, email, password_hash, password_algorithm,
                    password_version, password_last_changed, status, last_login,
                    activated_at, created_at, updated_at
                ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
                RETURNING *",
                &[
                    &user.id,
                    &user.username,
                    &user.email,
                    &user.password_hash,
                    &user.password_algorithm,
                    &user.password_version,
                    &user.password_last_changed,
                    &user.status.to_string().to_lowercase(),
                    &user.last_login,
                    &user.activated_at,
                    &user.created_at,
                    &user.updated_at,
                ],
            )
            .await?;

        Ok(drivers::database::schemas::user::User::from_row(&row)?.into())
    }

    async fn update(&self, user: &UserEntity) -> Result<UserEntity> {
        let client = self.pool.get().await?;
        let row = client
            .query_one(
                "UPDATE users SET
                    username = $2, email = $3, password_hash = $4,
                    password_algorithm = $5, password_version = $6,
                    password_last_changed = $7, status = $8, last_login = $9,
                    activated_at = $10, updated_at = $11
                WHERE id = $1
                RETURNING *",
                &[
                    &user.id,
                    &user.username,
                    &user.email,
                    &user.password_hash,
                    &user.password_algorithm,
                    &user.password_version,
                    &user.password_last_changed,
                    &user.status.to_string().to_lowercase(),
                    &user.last_login,
                    &user.activated_at,
                    &user.updated_at,
                ],
            )
            .await?;

        Ok(drivers::database::schemas::user::User::from_row(&row)?.into())
    }

    async fn delete(&self, id: &str) -> Result<()> {
        let client = self.pool.get().await?;
        client.execute("DELETE FROM users WHERE id = $1", &[&id]).await?;
        Ok(())
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<UserEntity>> {
        let client = self.pool.get().await?;
        let row = client
            .query_opt("SELECT * FROM users WHERE id = $1", &[&id])
            .await?;

        Ok(row.map(|r| drivers::database::schemas::user::User::from_row(&r).unwrap().into()))
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<UserEntity>> {
        let client = self.pool.get().await?;
        let row = client
            .query_opt("SELECT * FROM users WHERE email = $1", &[&email])
            .await?;

        Ok(row.map(|r| drivers::database::schemas::user::User::from_row(&r).unwrap().into()))
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<UserEntity>> {
        let client = self.pool.get().await?;
        let row = client
            .query_opt("SELECT * FROM users WHERE username = $1", &[&username])
            .await?;

        Ok(row.map(|r| drivers::database::schemas::user::User::from_row(&r).unwrap().into()))
    }

    async fn find_by_status(&self, status: &UserStatus) -> Result<Vec<UserEntity>> {
        let client = self.pool.get().await?;
        let status_str = status.to_string().to_lowercase();
        let rows = client
            .query("SELECT * FROM users WHERE status = $1", &[&status_str])
            .await?;

        Ok(rows
            .iter()
            .map(|r| drivers::database::schemas::user::User::from_row(r).unwrap().into())
            .collect())
    }

    async fn find_by_last_login_before(&self, date: &DateTime<Utc>) -> Result<Vec<UserEntity>> {
        let client = self.pool.get().await?;
        let rows = client
            .query(
                "SELECT * FROM users WHERE last_login < $1 OR last_login IS NULL",
                &[&date],
            )
            .await?;

        Ok(rows
            .iter()
            .map(|r| drivers::database::schemas::user::User::from_row(r).unwrap().into())
            .collect())
    }

    async fn find_inactive_users(&self, days: i64) -> Result<Vec<UserEntity>> {
        let client = self.pool.get().await?;
        let rows = client
            .query(
                "SELECT * FROM users WHERE last_login < NOW() - INTERVAL '$1 days' OR last_login IS NULL",
                &[&days],
            )
            .await?;

        Ok(rows
            .iter()
            .map(|r| drivers::database::schemas::user::User::from_row(r).unwrap().into())
            .collect())
    }

    async fn update_status(&self, id: &str, status: UserStatus) -> Result<UserEntity> {
        let client = self.pool.get().await?;
        let status_str = status.to_string().to_lowercase();
        let row = client
            .query_one(
                "UPDATE users SET status = $2, updated_at = NOW() WHERE id = $1 RETURNING *",
                &[&id, &status_str],
            )
            .await?;

        Ok(drivers::database::schemas::user::User::from_row(&row)?.into())
    }

    async fn update_password(
        &self,
        id: &str,
        password_hash: &str,
        algorithm: &str,
        version: i32,
    ) -> Result<UserEntity> {
        let client = self.pool.get().await?;
        let row = client
            .query_one(
                "UPDATE users SET
                    password_hash = $2,
                    password_algorithm = $3,
                    password_version = $4,
                    password_last_changed = NOW(),
                    updated_at = NOW()
                WHERE id = $1
                RETURNING *",
                &[&id, &password_hash, &algorithm, &version],
            )
            .await?;

        Ok(drivers::database::schemas::user::User::from_row(&row)?.into())
    }

    async fn update_last_login(&self, id: &str) -> Result<UserEntity> {
        let client = self.pool.get().await?;
        let row = client
            .query_one(
                "UPDATE users SET last_login = NOW(), updated_at = NOW() WHERE id = $1 RETURNING *",
                &[&id],
            )
            .await?;

        Ok(drivers::database::schemas::user::User::from_row(&row)?.into())
    }

    async fn exists_by_email(&self, email: &str) -> Result<bool> {
        let client = self.pool.get().await?;
        let row = client
            .query_one(
                "SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)",
                &[&email],
            )
            .await?;

        Ok(row.get(0))
    }

    async fn exists_by_username(&self, username: &str) -> Result<bool> {
        let client = self.pool.get().await?;
        let row = client
            .query_one(
                "SELECT EXISTS(SELECT 1 FROM users WHERE username = $1)",
                &[&username],
            )
            .await?;

        Ok(row.get(0))
    }
}