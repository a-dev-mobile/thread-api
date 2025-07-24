use crate::log_info;
use axum::{
    extract::{Extension, Query},
    response::IntoResponse,
};
use serde::Deserialize;
use sqlx::PgPool;

use crate::shared::utils::db::{execute_query, ResponseType};

#[derive(Deserialize, Debug)]
pub struct ToleranceParams {
    id: i32,
    #[serde(rename = "type")]
    type_: Option<String>,
}

pub async fn tolerance(Extension(pool): Extension<PgPool>, Query(params): Query<ToleranceParams>) -> impl IntoResponse {
    log_info!("Handling tolerance request with parameters: {:?}", params);

    let id = params.id;
    let type_ = params.type_.clone().unwrap_or_else(|| "f".to_string());

    // Определение SQL-запроса
    let query = "SELECT * FROM metric.get_tolerance($1, $2)";

    // Выполнение запроса с привязкой параметров
    match execute_query(
        &pool,
        query,
        move |q| q.bind(id).bind(type_),
        ResponseType::Multiple,
        None,
    )
    .await
    {
        Ok(json) => json.into_response(),
        Err(err) => err.into_response(),
    }
}
