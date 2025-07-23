use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub logging: LogConfig,
    pub postgres: PostgresConfig,
}

#[derive(Debug, Deserialize)]
pub struct LogConfig {
    pub level: String,
    pub format: String,
}

#[derive(Debug, Deserialize)]
pub struct PostgresConfig {
    pub timeout: u64,
    pub max_connections: u32,
    pub min_connections: u32,
    pub max_lifetime: u64,
    pub idle_timeout: u64,
}
