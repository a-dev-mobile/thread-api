use std::sync::Arc;

use crate::{
    features::{
        // Health feature
        health::{
            handler::{HealthHandler, HealthHandlerV1},
            service::{HealthService, HealthServiceImpl},
        },
    },
    shared::{database::service::PostgresService, setting::models::app_setting::AppSettings},
};

pub struct AppState {
    pub settings: Arc<AppSettings>,
    pub postgres_service: Arc<PostgresService>,

    // Health feature dependencies
    pub health_handler: Arc<dyn HealthHandler>,
    pub health_service: Arc<dyn HealthService>,
}

impl AppState {
    pub async fn new(settings: Arc<AppSettings>, postgres_service: Arc<PostgresService>) -> Self {
        // Получаем pool из postgres_service
        let pool = postgres_service.connection.pool().clone();

        // Создаем зависимости для health feature
        let health_service: Arc<dyn HealthService> = Arc::new(HealthServiceImpl::new(settings.clone(), pool.clone()));
        let health_handler: Arc<dyn HealthHandler> = Arc::new(HealthHandlerV1::new(health_service.clone()));

        Self {
            settings,
            postgres_service,

            health_handler,
            health_service,
        }
    }
}
