use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Deserialize, Clone)]
pub struct AppEnv {
    pub env: Env,
    pub server_port: u16,
    pub server_address: String,

    pub postgres_host: String,
    pub postgres_user: String,
    pub postgres_password: String,
    pub postgres_database: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
pub enum Env {
    Local,
    Dev,
    Production,
}

impl AppEnv {
    pub fn is_local(&self) -> bool {
        matches!(self.env, Env::Local)
    }

    pub fn is_dev(&self) -> bool {
        matches!(self.env, Env::Dev)
    }

    pub fn is_prod(&self) -> bool {
        matches!(self.env, Env::Production)
    }

    pub fn is_development(&self) -> bool {
        matches!(self.env, Env::Local | Env::Dev)
    }
}

impl FromStr for Env {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Env::Local),
            "dev" | "development" => Ok(Env::Dev),
            "prod" | "production" => Ok(Env::Production),
            _ => Err(format!("Unknown environment: {}", s)),
        }
    }
}

impl fmt::Display for Env {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Env::Local => "local",
            Env::Dev => "dev",
            Env::Production => "prod",
        };
        write!(f, "{}", s)
    }
}
