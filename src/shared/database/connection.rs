use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::sync::Arc;
use tracing::{debug, error, info};

use crate::shared::setting::models::app_setting::AppSettings;



#[derive(Clone)]
pub struct PostgresConnection {
    pool: Pool<Postgres>,
}

impl PostgresConnection {
    pub async fn new(settings: Arc<AppSettings>) -> Result<Self, sqlx::Error> {
        info!("Initializing PostgreSQL connection...");
        let env = &settings.env;
        let config_postgres = &settings.config.postgres;

        // Create connection string
        let connection_string = format!(
            "postgres://{}:{}@{}/{}",
            env.postgres_user, env.postgres_password, env.postgres_host, env.postgres_database
        );

        let pool = PgPoolOptions::new()
            .max_connections(config_postgres.max_connections)
            .min_connections(config_postgres.min_connections)
            .max_lifetime(std::time::Duration::from_secs(config_postgres.max_lifetime))
            .idle_timeout(std::time::Duration::from_secs(config_postgres.idle_timeout))
            .acquire_timeout(std::time::Duration::from_secs(config_postgres.timeout))
            .connect(&connection_string)
            .await?;

        // Test connection
        debug!("Executing test query on PostgreSQL");
        match sqlx::query("SELECT 1").execute(&pool).await {
            Ok(_) => info!("PostgreSQL connection successful"),
            Err(e) => {
                error!("Failed to connect to PostgreSQL: {}", e);
                return Err(e);
            }
        }

        Ok(Self { pool })
    }

    pub fn pool(&self) -> &Pool<Postgres> {
        &self.pool
    }

    pub async fn health_check(&self) -> Result<(), sqlx::Error> {
        sqlx::query("SELECT 1").execute(&self.pool).await?;
        Ok(())
    }
}
