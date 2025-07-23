use axum::{http::StatusCode, response::IntoResponse};

pub async fn test() -> impl IntoResponse {
    // info!("Handling test request");
    StatusCode::OK
}
