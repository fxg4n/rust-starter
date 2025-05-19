use sqlx::postgres::{PgPool, PgPoolOptions};
use anyhow::{Result, Context};
use tracing::{info, warn};
use crate::config::Config;
use crate::core::constants::database::*;

pub type DatabasePool = PgPool;

pub async fn create_pool(config: &Config) -> Result<DatabasePool> {
    info!("Initializing database connection pool...");

    let pool = PgPoolOptions::new()
        .min_connections(MIN_CONNECTIONS)
        .max_connections(MAX_CONNECTIONS)
        .max_lifetime(Some(MAX_LIFETIME))
        .acquire_timeout(ACQUIRE_TIMEOUT)
        .idle_timeout(Some(IDLE_TIMEOUT))
        .connect(&config.database.connection_string())
        .await
        .context("Failed to establish initial database connection")?;

    for attempt in 1..=CONNECTION_RETRY_ATTEMPTS {
        match check_database_connection(&pool).await {
            Ok(_) => break,
            Err(e) if attempt < CONNECTION_RETRY_ATTEMPTS => {
                warn!("Connection attempt {} failed: {}", attempt, e);
                tokio::time::sleep(CONNECTION_RETRY_DELAY).await;
            }
            Err(e) => return Err(e).context("Failed all database connection attempts"),
        }
    }

    info!(
        "Database connection pool initialized with {} max connections",
        MAX_CONNECTIONS
    );

    Ok(pool)
}

pub async fn check_database_connection(pool: &DatabasePool) -> Result<()> {
    tokio::time::timeout(
        STATEMENT_TIMEOUT,
        sqlx::query("SELECT 1").execute(pool)
    )
    .await
    .context("Database connection test timed out")?
    .map(|_| {
        info!("Database connection test successful");
        ()
    })
    .context("Database connection test failed")
}

pub async fn close_pool(pool: DatabasePool) {
    info!("Closing database connection pool...");
    pool.close().await;
    info!("Database connection pool closed");
}