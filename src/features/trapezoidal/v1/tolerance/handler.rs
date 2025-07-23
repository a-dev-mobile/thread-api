// src/routes/v1/trapezoidal/tolerance/trapezoidal_tolerance_handler.rs
use axum::{
    extract::{Extension, Query},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use regex::Regex;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use std::collections::HashSet;
use crate::{log_error, log_info};

#[derive(Deserialize, Debug)]
pub struct ToleranceParams {
    diameter: i32,
    pitch: f64,
}

#[derive(Serialize, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct ToleranceKey {
    number: i32,
    suffix: String,
}

#[derive(Serialize, Debug)]
struct ToleranceInfo {
    tolerance: String,
    formatted: String,
}

#[derive(Serialize, Debug)]
struct ToleranceResponse {
    male: Vec<ToleranceInfo>,
    female: Vec<ToleranceInfo>,
}

fn parse_tolerance(tolerance: &str) -> ToleranceKey {
    let re = Regex::new(r"(\d+)([a-zA-Z]+)").unwrap();
    if let Some(captures) = re.captures(tolerance) {
        ToleranceKey {
            number: captures[1].parse().unwrap_or(0),
            suffix: captures[2].to_string(),
        }
    } else {
        ToleranceKey {
            number: 0,
            suffix: tolerance.to_string(),
        }
    }
}

pub async fn handle(
    Extension(pool): Extension<PgPool>,
    Query(params): Query<ToleranceParams>,
) -> impl IntoResponse {
    log_info!(
        "Handling tolerance request for diameter: {}, pitch: {}",
        params.diameter, params.pitch
    );

    // Get column names to determine available tolerances
    let columns = match sqlx::query(
        r#"
        SELECT column_name
        FROM information_schema.columns
        WHERE table_schema = 'trapezoidal'
        AND table_name = 'main'
        AND (column_name LIKE 'es_d2_%_m' OR column_name LIKE 'es_d2_%_f')
        ORDER BY column_name;
    "#,
    )
    .fetch_all(&pool)
    .await
    {
        Ok(rows) => rows,
        Err(e) => {
            log_error!("Failed to fetch column information: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to fetch tolerance information"
                })),
            )
                .into_response();
        }
    };

    let mut male_tolerances = HashSet::new();
    let mut female_tolerances = HashSet::new();

    // Extract tolerance classes from column names
    for row in columns {
        let column_name: &str = row.get(0);
        if let Some(captures) = regex::Regex::new(r"es_d2_(\d+[a-z])_([mf])")
            .unwrap()
            .captures(column_name)
        {
            let tolerance = captures.get(1).unwrap().as_str();
            let thread_type = captures.get(2).unwrap().as_str();

            match thread_type {
                "m" => {
                    male_tolerances.insert(tolerance.to_string());
                }
                "f" => {
                    // Convert lowercase tolerance to uppercase for female threads
                    let upper_tolerance = tolerance
                        .chars()
                        .map(|c| {
                            if c.is_alphabetic() {
                                c.to_ascii_uppercase()
                            } else {
                                c
                            }
                        })
                        .collect::<String>();
                    female_tolerances.insert(upper_tolerance);
                }
                _ => {}
            }
        }
    }

    // Verify thread exists
    let thread_exists = sqlx::query!(
        "SELECT EXISTS(SELECT 1 FROM trapezoidal.main WHERE diameter = $1 AND pitch = $2) as exists",
        params.diameter,
        params.pitch
    )
    .fetch_one(&pool)
    .await
    .map_or(false, |row| row.exists.unwrap_or(false));

    if !thread_exists {
        return (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": format!("Thread with diameter {} and pitch {} not found", params.diameter, params.pitch)
            })),
        )
            .into_response();
    }

    // Convert to vectors and sort with custom sorting
    let mut male_vec: Vec<(ToleranceKey, ToleranceInfo)> = male_tolerances
        .into_iter()
        .map(|t| {
            let key = parse_tolerance(&t);
            let info = ToleranceInfo {
                tolerance: t.clone(),

                formatted: format!("Tr {} x {} - {}", params.diameter, params.pitch, t),
            };
            (key, info)
        })
        .collect();

    let mut female_vec: Vec<(ToleranceKey, ToleranceInfo)> = female_tolerances
        .into_iter()
        .map(|t| {
            let key = parse_tolerance(&t);
            let info = ToleranceInfo {
                tolerance: t.clone(),

                formatted: format!("Tr {} x {} - {}", params.diameter, params.pitch, t),
            };
            (key, info)
        })
        .collect();

    // Sort by ToleranceKey
    male_vec.sort_by(|a, b| a.0.cmp(&b.0));
    female_vec.sort_by(|a, b| a.0.cmp(&b.0));

    let response = ToleranceResponse {
        male: male_vec.into_iter().map(|(_, info)| info).collect(),
        female: female_vec.into_iter().map(|(_, info)| info).collect(),
    };

    (StatusCode::OK, Json(response)).into_response()
}
