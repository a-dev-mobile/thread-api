use axum::{http::StatusCode, Json};
use serde_json::Value;
use sqlx::{postgres::PgArguments, Column, PgPool, Row};
use crate::{log_error, log_info};

/// Тип ответа для функции `execute_query`.
pub enum ResponseType {
    Single,
    Multiple,
}

/// Выполняет SQL-запрос с заданными параметрами и возвращает результат в виде JSON с учетом опционального precision.
///
/// # Аргументы
///
/// * `pool` - Пул соединений с базой данных.
/// * `query` - SQL-запрос с плейсхолдерами ($1, $2, ...).
/// * `binds` - Функция для привязки параметров к запросу.
/// * `response_type` - Тип ответа: Single для одиночного объекта или Multiple для массива.
/// * `precision` - Опциональная точность форматирования числовых значений.
///
/// # Возвращает
///
/// * `Result<Json<Value>, (StatusCode, String)>` - Результат выполнения запроса или ошибка.
pub async fn execute_query<F>(
    pool: &PgPool,
    query: &str,
    binds: F,
    response_type: ResponseType,
    precision: Option<usize>,
) -> Result<Json<Value>, (StatusCode, String)>
where
    F: FnOnce(
        sqlx::query::Query<'_, sqlx::Postgres, PgArguments>,
    ) -> sqlx::query::Query<'_, sqlx::Postgres, PgArguments>,
{
    log_info!("Executing query: {}", query);
    // Привязка параметров к запросу
    let sql_query = binds(sqlx::query(query));

    // Выполнение запроса
    let rows = match sql_query.fetch_all(pool).await {
        Ok(r) => r,
        Err(e) => {
            log_error!("Error executing query: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Ошибка базы данных: {}", e),
            ));
        }
    };

    log_info!("Query executed successfully, received {} rows", rows.len());

    // Преобразование строк в JSON с учетом precision
    let results: Vec<Value> = rows.iter().map(|row| row_to_json(row, precision)).collect();

    match response_type {
        ResponseType::Single => {
            if results.len() == 1 {
                Ok(Json(results[0].clone()))
            } else {
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Ожидался один результат, но получено {}", results.len()),
                ))
            }
        }
        ResponseType::Multiple => Ok(Json(Value::Array(results))),
    }
}

/// Преобразует одну строку SQL-запроса в JSON-объект с учетом опционального precision.
///
/// # Аргументы
///
/// * `row` - Строка результата SQL-запроса.
/// * `precision` - Опциональная точность форматирования числовых значений.
///
/// # Возвращает
///
/// * `Value` - JSON-объект, представляющий строку.
fn row_to_json(row: &sqlx::postgres::PgRow, precision: Option<usize>) -> Value {
    let mut map = serde_json::Map::new();
    for column in row.columns() {
        let column_name = column.name();
        let value = if let Ok(val) = row.try_get::<Option<i32>, _>(column_name) {
            val.map(Value::from)
        } else if let Ok(val) = row.try_get::<Option<i64>, _>(column_name) {
            val.map(Value::from)
        } else if let Ok(val) = row.try_get::<Option<f64>, _>(column_name) {
            val.map(|v| {
                // Проверяем, является ли число целым
                if v.fract().abs() < 1e-10 {
                    // Преобразуем в i64, если возможно
                    let int_val = v as i64;
                    Value::from(int_val)
                } else if let Some(precision) = precision {
                    // Форматируем число с заданной точностью
                    let formatted = format!("{:.*}", precision, v);
                    // Преобразуем обратно в число с плавающей точкой
                    Value::from(formatted.parse::<f64>().unwrap_or(v))
                } else {
                    // Выводим как есть
                    Value::from(v)
                }
            })
        } else if let Ok(val) = row.try_get::<Option<bool>, _>(column_name) {
            val.map(Value::from)
        } else if let Ok(val) = row.try_get::<Option<String>, _>(column_name) {
            val.map(Value::from)
        } else {
            // Отлов ошибки обработки
            let error_message = format!(
                "Failed to process column '{}', data type: '{:?}'. The data type may not be supported.",

                column_name,
                column.type_info()
            );
            log_error!("{}", error_message);
            None
        };

        // Вставляем только если значение не равно null
        if let Some(v) = value {
            map.insert(column_name.to_string(), v);
        }
    }
    Value::Object(map)
}
