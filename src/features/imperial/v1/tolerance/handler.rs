use axum::{
    extract::{Extension, Query},
    http::StatusCode,
    response::IntoResponse,
    response::Response,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use tracing::error;

/// Структура для извлечения параметров запроса
#[derive(Deserialize, Debug)]
pub struct Params {
    diameter: String,
    tpi: f64,
}

/// Структура для представления данных из базы данных
#[derive(FromRow)]
struct DbModel {
    id: i64,
    diameter: String,
    diameter_2: f64,
    tpi: f64,
    class_f: String,
    class_m: String,
}

/// Вложенная структура для форматированных данных
#[derive(Serialize)]
struct FormattedSubModel {
    fractional: String,
    decimal: String,
}

/// Структура для информации о классе
#[derive(Serialize)]
struct ToleranceInfo {
    id: i64,
    series: String,
    formatted: FormattedSubModel,
}

/// Структура для ответа API
#[derive(Serialize)]
struct ResponseModel {
    female: Vec<ToleranceInfo>,
    male: Vec<ToleranceInfo>,
}

/// Обработчик для маршрута `/diameters`
pub async fn handle(Extension(pool): Extension<PgPool>, Query(params): Query<Params>) -> Response {
    // Выполнение запроса к базе данных через функцию
    let db_models = match fetch_db_models(&pool, &params).await {
        Ok(rows) => rows,
        Err(e) => {
            error!("Ошибка при выполнении запроса к базе данных: {}", e);
            return internal_server_error();
        }
    };

    // Преобразование записей из базы данных в модель ответа
    let female = db_models
        .iter()
        .map(|item| create_tolerance_info(item, &item.class_f))
        .collect::<Vec<_>>();

    let male = db_models
        .iter()
        .map(|item| create_tolerance_info(item, &item.class_m))
        .collect::<Vec<_>>();

    // Создаём экземпляр ResponseModel с собранными данными
    let response = ResponseModel { female, male };

    // Формирование успешного ответа
    (StatusCode::OK, Json(response)).into_response()
}

/// Функция для вызова PostgreSQL функции и получения данных из базы
async fn fetch_db_models(pool: &PgPool, params: &Params) -> Result<Vec<DbModel>, sqlx::Error> {
    sqlx::query_as::<_, DbModel>(
        "SELECT * FROM imperial.get_diameter_tolerance_data($1, $2)"
    )
    .bind(&params.diameter)
    .bind(params.tpi)
    .fetch_all(pool)
    .await
}

/// Вспомогательная функция для создания ToleranceInfo
fn create_tolerance_info(item: &DbModel, tolerance: &str) -> ToleranceInfo {
    let formatted = FormattedSubModel {
        fractional: format!("{} - {} - {}", item.diameter, item.tpi, tolerance),
        decimal: format!("{} - {} - {}", item.diameter_2, item.tpi, tolerance),
    };

    ToleranceInfo {
        id: item.id,
        series: tolerance.to_string(),
        formatted,
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
