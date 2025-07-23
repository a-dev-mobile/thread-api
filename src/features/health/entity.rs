use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Статус здоровья системы
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

impl std::fmt::Display for HealthStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HealthStatus::Healthy => write!(f, "healthy"),
            HealthStatus::Degraded => write!(f, "degraded"),
            HealthStatus::Unhealthy => write!(f, "unhealthy"),
        }
    }
}

/// Информация о компоненте системы
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentHealth {
    pub name: String,
    pub status: HealthStatus,
    pub message: Option<String>,
    pub response_time: Option<u64>, // в миллисекундах
}

/// Сущность проверки здоровья
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Health {
    pub status: HealthStatus,
    pub timestamp: DateTime<Utc>,
    pub uptime: u64, // в секундах
    pub version: String,
    pub components: Vec<ComponentHealth>,
}

impl Health {
    pub fn new(
        status: HealthStatus,
        uptime: u64,
        version: String,
        components: Vec<ComponentHealth>,
    ) -> Self {
        Self {
            status,
            timestamp: Utc::now(),
            uptime,
            version,
            components,
        }
    }

    /// Создает базовую проверку здоровья
    pub fn basic(uptime: u64, version: String) -> Self {
        Self::new(HealthStatus::Healthy, uptime, version, vec![])
    }

    /// Определяет общий статус на основе компонентов
    pub fn calculate_overall_status(&mut self) {
        if self.components.is_empty() {
            self.status = HealthStatus::Healthy;
            return;
        }

        let unhealthy_count = self.components.iter()
            .filter(|c| c.status == HealthStatus::Unhealthy)
            .count();

        let degraded_count = self.components.iter()
            .filter(|c| c.status == HealthStatus::Degraded)
            .count();

        if unhealthy_count > 0 {
            self.status = HealthStatus::Unhealthy;
        } else if degraded_count > 0 {
            self.status = HealthStatus::Degraded;
        } else {
            self.status = HealthStatus::Healthy;
        }
    }
}