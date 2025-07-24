use crate::{log_error, log_info};
use axum::{
    extract::{Json, State},
    http::HeaderMap,
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::Value;
use sqlx::PgPool;

use crate::{features::error_reports::models::ErrorReport, shared::utils::http::get_client_ip_from_headers};

#[axum::debug_handler]
pub async fn create_error_report(
    State(pool): State<PgPool>,
    headers: HeaderMap,
    Json(payload): Json<Value>,
) -> impl IntoResponse {
    let client_ip = get_client_ip_from_headers(&headers);

    log_info!("Received error report from {}: {}", client_ip, payload);

    let result = sqlx::query_as!(
        ErrorReport,
        r#"
        INSERT INTO analytics.error_reports (json_data, client_ip)
        VALUES ($1, $2)
        RETURNING id, timestamp, json_data, client_ip
        "#,
        payload,
        client_ip
    )
    .fetch_one(&pool)
    .await;

    match result {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => {
            log_error!("Failed to insert error report: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Не удалось сохранить отчет об ошибке"})),
            )
                .into_response()
        }
    }
}
