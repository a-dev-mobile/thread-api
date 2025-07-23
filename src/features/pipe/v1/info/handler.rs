// src/routes/v1/pipe/info/handler_pipe_info.rs
use crate::{
    analytics::db::handle_thread_analytics,
    shared::error::AppError,
    features::pipe::v1::{
        common::{
            db::ThreadDataService,
            models::model_pipe_db::ModelPipeDB,
        },
        info::models::{request_pipe_info::RequestPipeInfo, response_pipe_info::ResponsePipeInfo},
    },
};
use axum::{
    extract::Extension, http::StatusCode, response::IntoResponse, response::Response, Json,
};
use sqlx::query_as;
use sqlx::PgPool;

use crate::log_error;

use axum::extract::Query;

const QUERY_PIPE: &str = "
SELECT id,
       designation,
       designation_2,
       thread_pitch,
       thread_per,
       class_name,
       ex_major_dia_max,
       ex_major_dia_min,
       ex_pitch_diam_max,
       ex_pitch_diam_min,
       ex_minor_dia_max,
       in_minor_dia_min,
       in_minor_dia_max,
       in_pitch_diam_min,
       in_pitch_diam_max,
       in_major_dia_min,
       in_tap_drill
FROM pipe.main
where id = $1;
";

pub async fn handle(
    Extension(pool): Extension<PgPool>,
    Query(request): Query<RequestPipeInfo>,
) -> Response {
    // Выполнение запроса к базе данных
    let db_records = query_as::<_, ModelPipeDB>(QUERY_PIPE)
        .bind(request.id)
        .fetch_one(&pool)
        .await;

    let db_records = match db_records {
        Ok(records) => records,
        Err(e) => {
            log_error!("Ошибка при выполнении запроса к базе данных: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Внутренняя ошибка сервера" })),
            )
                .into_response();
        }
    };
    let response = ResponsePipeInfo::from_data(db_records, &request);


          // Clone pool and designation for background task
          let pool_clone = pool.clone();
          let designation_clone = response.designation1.clone();
            // Spawn background task for analytics
            tokio::spawn(async move {
                handle_thread_analytics(pool_clone, designation_clone).await;
            });


    (StatusCode::OK, Json(response)).into_response()

}
