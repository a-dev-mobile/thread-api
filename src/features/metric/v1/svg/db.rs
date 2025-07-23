use crate::features::metric::v1::svg::params::SvgParams;
use crate::shared::utils::db::{execute_query, ResponseType};
use axum::http::StatusCode;
use axum::Json;
use serde_json::Value;
use sqlx::Pool;
use sqlx::Postgres;

pub async fn fetch_thread_info_from_db(
    pool: &Pool<Postgres>,
    params: &SvgParams,
) -> Result<Json<Value>, (StatusCode, String)> {
    let query = "SELECT * FROM metric.get_info($1, $2, $3, $4, $5, $6)";
    execute_query(
        pool,
        query,
        |q| {
            q.bind(params.diameter)
                .bind(params.pitch)
                .bind(params.type_.clone())
                .bind(params.tolerance.clone())
                .bind(params.language.clone())
                .bind(params.units.clone())
        },
        ResponseType::Single,
        Some(params.precision),
    )
    .await
}
