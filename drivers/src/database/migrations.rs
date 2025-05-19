use anyhow::{Result, Context};
use sqlx::PgPool;
use tracing::{info,error};

pub async fn run_migrations(pool: &PgPool) -> Result<()> {
    info!("Running database migrations...");

    match sqlx::migrate!("./migrations").run(pool).await {
        Ok(_) => {
            info!("Database migrations completed successfully");
            Ok(())
        }
        Err(e) => {
            error!("Failed to run migrations: {}", e);
            Err(e).context("Database migration failed")
        }
    }
}

pub async fn revert_last_migration(pool: &PgPool) -> Result<()> {
    info!("Reverting last database migration...");

    let migrator = sqlx::migrate!("./migrations");
    let current_version = migrator.migrations.len() as i64;
    let target_version = if current_version > 0 { current_version - 1 } else { 0 };

    match migrator.undo(pool, target_version).await {
        Ok(_) => {
            info!("Last migration reverted successfully");
            Ok(())
        }
        Err(e) => {
            error!("Failed to revert migration: {}", e);
            Err(e).context("Failed to revert last migration")
        }
    }
}