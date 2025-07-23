use axum::{
    extract::{Extension, Query},
    http::StatusCode,
    response::IntoResponse,
    response::Response,
    Json,
};

use sqlx::PgPool;
use tracing::error;

use crate::{
    analytics::db::handle_thread_analytics,
    shared::enums::{Language, ThreadType, Unit},

    features::imperial::v2::info::mappers::ImperialInfoMapper,
};

use super::models::{db::ModelV2ImperialDB, request::RequestV2ImperialInfo};

const QUERY_IMPERIAL: &str = "
SELECT * FROM imperial.main WHERE id = $1
";

pub async fn handle(
    Extension(pool): Extension<PgPool>,
    Query(request): Query<RequestV2ImperialInfo>,
) -> impl IntoResponse {
    let is_male = matches!(request.type_, ThreadType::Male);

    // Build query based on thread type
    let query = if is_male {
        "SELECT * FROM imperial.main WHERE diameter = $1 AND tpi = $2 AND class_m = $3"
    } else {
        "SELECT * FROM imperial.main WHERE diameter = $1 AND tpi = $2 AND class_f = $3"
    };

    let db_model = match  sqlx::query_as::<_, ModelV2ImperialDB>(query)
    .bind(&request.diameter)
    .bind(request.tpi)
    .bind(&request.series)
    .fetch_one(&pool)
    .await
    {
        Ok(record) => record,
        Err(e) => {
            error!("Database query error: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": format!("Thread with diameter: {}, TPI: {}, class: {} not found",
                    request.diameter, request.tpi, request.series)
            })).into_response());
        }
    };

   
    // Generate designation for analytics
    let designation = ImperialInfoMapper::generate_designation1(&db_model, &request.type_);

    // Clone pool and designation for background task
    let pool_clone = pool.clone();
    let designation_clone = designation.clone();

    // Spawn background task for analytics
    tokio::spawn(async move {
        handle_thread_analytics(pool_clone, designation_clone).await;
    });

    // Map database record to response
    let response = ImperialInfoMapper::from_data(db_model, &request);


    (StatusCode::OK, Json(response).into_response())
}
