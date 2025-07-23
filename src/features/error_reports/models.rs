use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ErrorReport {
    pub id: i32,
    pub timestamp: Option<DateTime<Utc>>,
    pub json_data: Value,
    pub client_ip: String,
}
