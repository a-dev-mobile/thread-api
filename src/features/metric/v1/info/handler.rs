use crate::log_info;
use axum::{
    extract::{Extension, Query},
    response::IntoResponse,
};
use serde::Deserialize;
use sqlx::PgPool;

use crate::shared::utils::db::{execute_query, ResponseType};

#[derive(Deserialize, Debug)]
pub struct InfoParams {
    diameter: f64,
    pitch: f64,
    #[serde(rename = "type")]
    type_: String,
    tolerance: String,
    language: String,
    units: String,
    precision: Option<usize>,
}

pub async fn info(Extension(pool): Extension<PgPool>, Query(params): Query<InfoParams>) -> impl IntoResponse {
    log_info!("Processing info request with parameters: {:?}", params);

    let query = "SELECT * FROM metric.get_info($1, $2, $3, $4, $5, $6)";

    match execute_query(
        &pool,
        query,
        move |q| {
            q.bind(params.diameter)
                .bind(params.pitch)
                .bind(params.type_)
                .bind(params.tolerance)
                .bind(params.language)
                .bind(params.units)
        },
        ResponseType::Single,
        params.precision, // Passing optional precision
    )
    .await
    {
        Ok(json) => json.into_response(),
        Err(err) => err.into_response(),
    }
}
