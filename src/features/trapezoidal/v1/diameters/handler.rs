use crate::features::trapezoidal::v1::diameters::models::{DbModel, ResponseModel};
use axum::{
    extract::Extension, http::StatusCode, response::IntoResponse, response::Response, Json,
};
use sqlx::PgPool;
use crate::log_error;

const DIAMETERS_QUERY: &str = "
SELECT DISTINCT ON (diameter, pitch)
    id,
    diameter,
    pitch

FROM
    trapezoidal.main
ORDER BY diameter, pitch, id;
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

    // Преобразование записей из базы данных в модель ответа
    let response: Vec<ResponseModel> = db_records
        .into_iter()
        .map(|record| ResponseModel {
            diameter: record.diameter.to_string(),
            pitch: record.pitch.to_string(),
            designation: format!("Tr {} x {}", record.diameter, record.pitch),
        })
        .collect();

    // Формирование успешного ответа

    (StatusCode::OK, Json(response)).into_response()
}

async fn fetch_diameters(pool: &PgPool) -> Result<Vec<DbModel>, sqlx::Error> {
    sqlx::query_as::<_, DbModel>(DIAMETERS_QUERY)
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
