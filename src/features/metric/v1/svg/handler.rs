use crate::analytics::db::handle_thread_analytics;
use crate::features::metric::models::ThreadInfo;
use crate::features::metric::v1::svg::coords::initialize;
use crate::features::metric::v1::svg::db::fetch_thread_info_from_db;
use crate::features::metric::v1::svg::text_generation::generate_svg_texts;

use axum::{
    extract::{Extension, Query},
    http::{
        header::{HeaderMap, HeaderValue, CONTENT_TYPE},
        StatusCode,
    },
    response::IntoResponse,
};

use crate::features::metric::v1::svg::params::SvgParams;
use crate::{log_error, log_info};
use sqlx::PgPool;
use std::path::PathBuf;
use tokio::fs;

pub async fn svg(Extension(pool): Extension<PgPool>, Query(params): Query<SvgParams>) -> impl IntoResponse {
    // Fetching thread information
    let thread_info_json = match fetch_thread_info_from_db(&pool, &params).await {
        Ok(val) => val.0,
        Err(err) => return err.into_response(),
    };

    // Deserializing JSON into the ThreadInfo struct
    let thread_info: ThreadInfo = match serde_json::from_value(thread_info_json.clone()) {
        Ok(info) => info,
        Err(err) => {
            log_info!(
                "Error deserializing JSON: {}",
                serde_json::to_string(&thread_info_json).unwrap_or_else(|_| "invalid JSON".to_string())
            );
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error deserializing: {}", err),
            )
                .into_response();
        }
    };

    log_info!("Thread information received: {:?}", thread_info);

    if params.show_dimensions {
        // Вызов функции add_or_increment_thread в фоне через модуль analytics
        let designation_clone = thread_info.designation.clone();
        let pool_clone = pool.clone();
        tokio::spawn(async move {
            handle_thread_analytics(pool_clone, designation_clone).await;
        });
    }

    // Loading SVG template based on type and theme
    let result_load_svg_template = match load_svg_template(&params.type_, &params.theme).await {
        Ok(template) => template,
        Err(err) => {
            log_error!("Error loading SVG template: {}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to load SVG template.".to_string(),
            )
                .into_response();
        }
    };
    let coords = initialize(&params.type_);

    // Generating text elements based on theme and language
    let svg_texts = generate_svg_texts(
        &thread_info,
        &params.type_,
        &coords,
        &params.theme,
        &params.language,
        params.show_dimensions,
    );

    // Inserting text elements into the SVG
    if let Some(index) = result_load_svg_template.rfind("</svg>") {
        let mut updated_svg = result_load_svg_template.clone();
        updated_svg.insert_str(index, &svg_texts);

        let content = updated_svg.into_bytes();
        send_success_response(content)
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Invalid SVG format: closing </svg> tag not found.".to_string(),
        )
            .into_response()
    }
}

fn send_success_response(content: Vec<u8>) -> axum::http::Response<axum::body::Body> {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("image/svg+xml"));
    (StatusCode::OK, headers, content).into_response()
}

async fn load_svg_template(type_: &str, theme: &str) -> Result<String, std::io::Error> {
    // Forming the file name
    let file_name = format!("metric-thread-{}-{}.svg", type_.to_lowercase(), theme.to_lowercase());
    let file_path = PathBuf::from("./static/svg").join(file_name);
    fs::read_to_string(file_path).await
}
