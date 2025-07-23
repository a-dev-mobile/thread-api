use super::{app_config::AppConfig, app_env::AppEnv};

#[derive(Debug)]
pub struct AppSettings {
    pub config: AppConfig,
    pub env: AppEnv,
}
