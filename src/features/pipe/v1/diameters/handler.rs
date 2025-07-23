use crate::features::pipe::v1::common::models::model_pipe_db::ModelPipeDB;
use super::models::ResponsePipeDiameters;
use axum::{
    extract::Extension, http::StatusCode, response::IntoResponse, response::Response, Json,
};
use sqlx::PgPool;
use crate::log_error;


const QUERY_PIPE: &str = "
SELECT
    id,
    designation,
    designation_2,
    thread_pitch,
    thread_per,
    class_name,
    ex_major_dia_max,
    NULL::double precision as ex_major_dia_min,
    NULL::double precision as ex_pitch_diam_max,
    NULL::double precision as ex_pitch_diam_min,
    NULL::double precision as ex_minor_dia_max,
    NULL::double precision as in_minor_dia_min,
    NULL::double precision as in_minor_dia_max,
    NULL::double precision as in_pitch_diam_min,
    NULL::double precision as in_pitch_diam_max,
    in_major_dia_min,
    NULL::double precision as in_tap_drill
FROM pipe.main
ORDER BY designation ASC;
";


pub async fn handle(Extension(pool): Extension<PgPool>) -> Response {
    // Выполнение запроса к базе данных
    let db_records = match fetch_diameters(&pool).await {
        Ok(records) => records,
        Err(e) => {
            log_error!("Ошибка при выполнении запроса к базе данных: {}", e);
            return internal_server_error();
        }
    };

    let response = ResponsePipeDiameters::from(db_records);
    
    (StatusCode::OK, Json(response)).into_response()
}

async fn fetch_diameters(pool: &PgPool) -> Result<Vec<ModelPipeDB>, sqlx::Error> {
    sqlx::query_as::<_, ModelPipeDB>(QUERY_PIPE)
        .fetch_all(pool)
        .await
}

fn internal_server_error() -> Response {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({ "error": "Внутренняя ошибка сервера" })),
    )
        .into_response()
}
