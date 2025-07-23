use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use serde_json::Value;

use super::types::AppError;


#[derive(Serialize)]
struct ErrorResponse {
    code: &'static str,
    message: String,
    #[serde(skip_serializing_if = "is_empty_details")]
    details: Value,
    timestamp: String,
}

fn is_empty_details(details: &Value) -> bool {
    details.as_object().is_none_or(|obj| obj.is_empty())
}
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let error_response = ErrorResponse {
            code: self.error_code(),
            message: self.to_string(),
            details: self.details(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        };

        (self.status_code(), Json(error_response)).into_response()
    }
}
