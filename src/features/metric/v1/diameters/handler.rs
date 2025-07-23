use axum::{
    extract::{Extension, Query},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use sqlx::PgPool;
use crate::log_info;

use crate::shared::utils::db::{execute_query, ResponseType};



#[derive(Deserialize, Debug)]
pub struct DiameterParams {
    order: Option<String>,
}

pub async fn diameters(
    Extension(pool): Extension<PgPool>,
    Query(params): Query<DiameterParams>,
) -> impl IntoResponse {
    log_info!("Handling diameters request with parameters: {:?}", params);
    // Clone the order to own the String
    let order = params.order.clone().unwrap_or_else(|| "asc".to_string());

    // Валидация параметра order
    if order != "asc" && order != "desc" {
        return (
            StatusCode::BAD_REQUEST,
            "Invalid parameter 'order'. Use 'asc' or 'desc'.",
        )
            .into_response();
    }

    // Определение SQL-запроса
    let query = "SELECT * FROM metric.get_diameters($1)";

    // Выполнение запроса с привязкой параметра
    match execute_query(
        &pool,
        query,
        move |q| q.bind(order),
        ResponseType::Multiple,
        None,
    )
    .await
    {
        Ok(json) => json.into_response(),
        Err(err) => err.into_response(),
    }
}
