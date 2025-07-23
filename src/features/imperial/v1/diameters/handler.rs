use axum::{
    extract::Extension, http::StatusCode, response::IntoResponse, response::Response, Json,
};
use serde::Serialize;
use sqlx::{FromRow, PgPool};
use tracing::error;

/// Модель данных из базы данных
#[derive(Serialize, FromRow)]
struct DiameterModelDb {
    id: i64,
    diameter: String,
    diameter_2: f64,
    tpi: f64,
    series_designation: String,
}

/// Вложенная структура для форматированных данных
#[derive(Serialize)]
struct FormattedSubModel {
    fractional: String,
    decimal: String,
}

/// Структура для ответа API
#[derive(Serialize)]
struct ResponseModel {
    id: i64,
    formatted: FormattedSubModel,
    series: String,
    tpi: String,
    diameter: String,
}

/// Обработчик для маршрута `/diameters`
pub async fn handle(Extension(pool): Extension<PgPool>) -> Response {
    // Выполнение запроса к базе данных через функцию
    let db_records = match fetch_diameters(&pool).await {
        Ok(records) => records,
        Err(e) => {
            error!("Ошибка при выполнении запроса к базе данных: {}", e);
            return internal_server_error();
        }
    };

    // Преобразование записей из базы данных в модель ответа
    let response: Vec<ResponseModel> = db_records.into_iter().map(transform_record).collect();

    // Сортировка уже выполнена в функции базы данных, дополнительная сортировка не нужна

    // Формирование успешного ответа
    (StatusCode::OK, Json(response)).into_response()
}

/// Функция для вызова PostgreSQL функции и получения данных из базы
async fn fetch_diameters(pool: &PgPool) -> Result<Vec<DiameterModelDb>, sqlx::Error> {
    sqlx::query_as::<_, DiameterModelDb>("SELECT * FROM imperial.get_unique_diameters()")
        .fetch_all(pool)
        .await
}

/// Функция для преобразования записи из базы данных в модель ответа
fn transform_record(record: DiameterModelDb) -> ResponseModel {
    ResponseModel {
        id: record.id,
        formatted: FormattedSubModel {
            fractional: format!("{} - {}", record.diameter, record.tpi),
            decimal: format!("{} - {}", record.diameter_2, record.tpi),
        },
        series: record.series_designation,
        diameter: record.diameter,
        tpi: record.tpi.to_string(),
    }
}

/// Функция для формирования ответа с внутренней ошибкой сервера
fn internal_server_error() -> Response {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({ "error": "Внутренняя ошибка сервера" })),
    )
        .into_response()
}
