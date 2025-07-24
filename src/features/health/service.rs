use sqlx::PgPool;
use std::sync::Arc;
use std::time::Instant;
use tokio::time::{timeout, Duration};

use crate::shared::setting::models::app_setting::AppSettings;

use super::entity::{ComponentHealth, Health, HealthStatus};

/// Трейт сервиса проверки здоровья
#[async_trait::async_trait]
pub trait HealthService: Send + Sync {
    async fn get_health(&self) -> Health;
}

/// Реализация сервиса проверки здоровья
pub struct HealthServiceImpl {
    settings: Arc<AppSettings>,
    pool: PgPool,
    start_time: Instant,
}

impl HealthServiceImpl {
    pub fn new(settings: Arc<AppSettings>, pool: PgPool) -> Self {
        Self {
            settings,
            pool,
            start_time: Instant::now(),
        }
    }

    /// Проверяет подключение к базе данных
    async fn check_database(&self) -> ComponentHealth {
        let start = Instant::now();

        let result = timeout(
            Duration::from_millis(5000), // 5 секунд таймаут
            sqlx::query("SELECT 1").execute(&self.pool),
        )
        .await;

        let response_time = start.elapsed().as_millis() as u64;

        match result {
            Ok(Ok(_)) => ComponentHealth {
                name: "database".to_string(),
                status: if response_time > 1000 {
                    HealthStatus::Degraded
                } else {
                    HealthStatus::Healthy
                },
                message: Some("Connected to PostgreSQL".to_string()),
                response_time: Some(response_time),
            },
            Ok(Err(e)) => ComponentHealth {
                name: "database".to_string(),
                status: HealthStatus::Unhealthy,
                message: Some(format!("Database error: {}", e)),
                response_time: Some(response_time),
            },
            Err(_) => ComponentHealth {
                name: "database".to_string(),
                status: HealthStatus::Unhealthy,
                message: Some("Database connection timeout".to_string()),
                response_time: Some(response_time),
            },
        }
    }

    /// Получает время работы приложения
    fn get_uptime(&self) -> u64 {
        self.start_time.elapsed().as_secs()
    }

    /// Получает версию приложения
    fn get_version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }
}

#[async_trait::async_trait]
impl HealthService for HealthServiceImpl {
    /// Полная проверка здоровья со всеми компонентами
    async fn get_health(&self) -> Health {
        let mut components = vec![];

        // Проверяем базу данных
        components.push(self.check_database().await);

        // Можно добавить другие проверки:
        // - Redis
        // - External APIs
        // - File system
        // - Memory usage

        let mut health = Health::new(HealthStatus::Healthy, self.get_uptime(), self.get_version(), components);

        health.calculate_overall_status();
        health
    }
}
