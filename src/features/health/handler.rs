use axum::{
    http::StatusCode,
    response::{IntoResponse, Json as JsonResponse, Response},
};
use std::sync::Arc;

use super::{entity::HealthStatus, service::HealthService};

/// Трейт обработчика health проверок
#[async_trait::async_trait]
pub trait HealthHandler: Send + Sync {
    async fn get_health(&self) -> Response;
}

/// Реализация обработчика health v1
pub struct HealthHandlerV1 {
    service: Arc<dyn HealthService>,
}

impl HealthHandlerV1 {
    pub fn new(service: Arc<dyn HealthService>) -> Self {
        Self { service }
    }

    /// Определяет HTTP статус код на основе статуса здоровья
    fn status_to_http_code(status: &HealthStatus) -> StatusCode {
        match status {
            HealthStatus::Healthy => StatusCode::OK,
            HealthStatus::Degraded => StatusCode::OK, // 200, но с предупреждением
            HealthStatus::Unhealthy => StatusCode::SERVICE_UNAVAILABLE, // 503
        }
    }
}

#[async_trait::async_trait]
impl HealthHandler for HealthHandlerV1 {
    /// GET /health - полная проверка здоровья
    async fn get_health(&self) -> Response {
        let health = self.service.get_health().await;
        let status_code = Self::status_to_http_code(&health.status);

        (status_code, JsonResponse(health)).into_response()
    }
}
