use super::models::app_env::{AppEnv, Env};
use std::env;
use std::str::FromStr;

impl AppEnv {
    pub fn new() -> AppEnv {
        let env = Env::from_str(&get_env_var("ENV")).expect("Unknown environment");
        let server_port = get_env_var("SERVER_PORT").parse().expect("PORT must be a number");
        let server_address = get_env_var("SERVER_ADDRESS");

        let postgres_user = get_env_var("POSTGRES_USER");
        let postgres_password = get_env_var("POSTGRES_PASSWORD");
        let postgres_host = get_env_var("POSTGRES_HOST");
        let postgres_database = get_env_var("POSTGRES_DATABASE");

        AppEnv {
            env,
            server_port,
            server_address,
            postgres_host,
            postgres_user,
            postgres_password,
            postgres_database,
        }
    }
}

impl Default for AppEnv {
    fn default() -> Self {
        Self::new()
    }
}

fn get_env_var(name: &str) -> String {
    env::var(name).unwrap_or_else(|_| panic!("ENV -> {} is not set", name))
}
