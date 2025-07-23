// src/routes/v1/trapezoidal/info/handler_trapezoidal_info.rs

use crate::{
    analytics::db::handle_thread_analytics, shared::error::AppError, features::trapezoidal::{
        common::{
            calculators::{calculate_additional_info, calculate_diameter_info, calculate_main_info, get_thread_info},
            db::ThreadDataService,
        },
        v1::info::{
            models::{request::RequestTrapezoidalInfo, response::ResponseTrapezoidalInfo},
        },
    }
};

use axum::{
    extract::{Extension, Query},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use crate::log_error;

pub async fn handle(
    Extension(pool): Extension<PgPool>,
    Query(params): Query<RequestTrapezoidalInfo>,
) -> Result<impl IntoResponse, AppError> {
    // Initialize database service
    let db_service = ThreadDataService::new(pool.clone());  // Clone pool for analytics use

    // Fetch thread data using the core service
    let thread_data = match db_service
        .fetch_thread_data(
            params.diameter,
            params.pitch,
            params.type_thread,
            &params.tolerance,
        )
        .await
    {
        Ok(data) => data,
        Err(e) => {
            log_error!("Database query error: {}", e);
            let error_message = if e.to_string().contains("no rows") {
                "No thread specifications found for the given diameter and pitch"
            } else {
                "An error occurred while retrieving thread specifications"
            };
            return Ok((
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "error": error_message,
                    "details": e.to_string()
                })),
            )
                .into_response());
        }
    };

    // Get thread description and designation
    let (_, designation) = get_thread_info(
        params.language,
        params.type_thread,
        params.diameter,
        params.pitch,
        &params.tolerance,
    );

    // Clone designation for analytics
    let designation_clone = designation.clone();
    
    // Clone pool for background task
    let pool_clone = pool.clone();

    // Spawn background task for analytics
    tokio::spawn(async move {
        handle_thread_analytics(pool_clone, designation_clone).await;
    });

    // Calculate response data
    let (description, designation) = get_thread_info(
        params.language,
        params.type_thread,
        params.diameter,
        params.pitch,
        &params.tolerance,
    );

    let main_info = calculate_main_info(
        params.diameter,
        params.pitch,
        thread_data.other_dimensions.h4_h3,
        params.tolerance.clone(),
        params.type_thread,
        params.language,
        params.units,
        params.precision,
    );

    let diameter_info = calculate_diameter_info(
        params.language,
        params.type_thread,
        params.units,
        params.precision,
        &thread_data.basic_diameters,
        &thread_data.tolerances,
    );

    // Create the response with all thread info
    let response = ResponseTrapezoidalInfo {
        description,
        designation,
        main_info,
        diameter_info,
        additional_info: calculate_additional_info(
            params.language,
            params.type_thread,
            params.units,
            params.precision,
            &thread_data,
        ),
    };

    Ok((StatusCode::OK, Json(response)).into_response())
}
