use crate::log_info;
use axum::{
    extract::{Extension, Query},
    response::IntoResponse,
};
use serde::Deserialize;
use sqlx::PgPool;

use crate::shared::utils::db::{execute_query, ResponseType};

#[derive(Deserialize, Debug)]
pub struct PitchParams {
    diameter: f64,
    language: Option<String>,
}

pub async fn pitch(Extension(pool): Extension<PgPool>, Query(params): Query<PitchParams>) -> impl IntoResponse {
    log_info!("Handling pitch request with parameters: {:?}", params);

    let diameter = params.diameter;
    // Clone the language to own the String
    let language = params.language.clone().unwrap_or_else(|| "en".to_string());

    // Определение SQL-запроса
    let query = "SELECT * FROM metric.get_pitch($1, $2)";

    match execute_query(
        &pool,
        query,
        move |q| q.bind(diameter).bind(language),
        ResponseType::Multiple, // Явно указываем массив
        None,
    )
    .await
    {
        Ok(json) => json.into_response(),
        Err(err) => err.into_response(),
    }
}
