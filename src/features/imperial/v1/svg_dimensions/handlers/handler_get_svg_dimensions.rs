use axum::body::Body;

use crate::log_error;
use axum::http::{Response, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use axum::{extract::Query, Extension};
use sqlx::PgPool;

use crate::features::imperial::v1::svg_dimensions::models::RequestSvgDimension;
use crate::services::svg::enums::{FontFamily, FontWeight, TextAnchor};
use crate::shared::enums::{Language, ThreadStandard, ThreadType, Unit};
use crate::shared::error::AppError;

use crate::features::imperial::v1::info::models::DbModel;
use crate::shared::utils::number::NumberFormatter;

use crate::services::svg::{SvgService, SvgText, TextOptionsGenerator};

pub async fn handle(
    Extension(pool): Extension<PgPool>,
    Query(params): Query<RequestSvgDimension>,
) -> Result<Response<Body>, AppError> {
    let is_male = matches!(params.thread_type, ThreadType::Male);

    // Build query based on thread type
    let query = if is_male {
        "SELECT * FROM imperial.main WHERE diameter = $1 AND tpi = $2 AND class_m = $3"
    } else {
        "SELECT * FROM imperial.main WHERE diameter = $1 AND tpi = $2 AND class_f = $3"
    };

    // Fetch thread data from database
    let thread_data = match sqlx::query_as::<_, DbModel>(query)
        .bind(&params.diameter)
        .bind(params.tpi)
        .bind(&params.tolerance)
        .fetch_one(&pool)
        .await
    {
        Ok(data) => data,
        Err(e) => {
            log_error!("Database query error: {}", e);
            let error_message = if e.to_string().contains("no rows") {
                "No thread specifications found for the given diameter and TPI"
            } else {
                "An error occurred while retrieving thread specifications"
            };
            return Ok((
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "error": error_message,
                    "details": e.to_string()
                })),
            )
                .into_response());
        }
    };

    let svg_service = SvgService::new("./static/svg");

    let mut svg_content = svg_service
        .load_template(ThreadStandard::Imperial, params.thread_type, params.theme)
        .await?;

    // Create text options
    let text_option_30_90_center_normal =
        svg_service.create_custom_text_options(30.0, -90.0, TextAnchor::Middle, FontWeight::Normal, FontFamily::Arial);
    let text_option_20_0_center_normal =
        svg_service.create_custom_text_options(20.0, 0.0, TextAnchor::Middle, FontWeight::Normal, FontFamily::Arial);
    let text_option_30_0_start_normal =
        svg_service.create_custom_text_options(30.0, 0.0, TextAnchor::Start, FontWeight::Normal, FontFamily::Arial);
    let text_option_40_0_start_normal =
        svg_service.create_custom_text_options(40.0, 0.0, TextAnchor::Start, FontWeight::Normal, FontFamily::Arial);
    let text_option_40_0_center_normal =
        svg_service.create_custom_text_options(40.0, 0.0, TextAnchor::Middle, FontWeight::Normal, FontFamily::Arial);
    let text_option_40_0_end_normal =
        svg_service.create_custom_text_options(40.0, 0.0, TextAnchor::End, FontWeight::Normal, FontFamily::Arial);
    let text_options_diameters_avg =
        svg_service.create_custom_text_options(12.0, -90.0, TextAnchor::Middle, FontWeight::Normal, FontFamily::Arial);

    // Get language-specific thread labels
    let (external_thread, internal_thread, _avg_label, min_label, _max_label) = match params.language {
        Language::Ru => ("Наружная резьба", "Внутренняя резьба", "сред.", "мин.", "макс."),
        Language::En => ("External thread", "Internal thread", "avg.", "min.", "max."),
    };

    // Format pitch as TPI
    let pitch_display = format!("{} TPI", params.tpi);

    // Calculate H (basic thread height) from TPI
    let h = 0.866025 / params.tpi; // H = 0.866025 / TPI

    // Helper function to convert and format values
    let convert_value = |value: f64| -> String {
        let converted = NumberFormatter::convert_and_round(
            value,
            &Unit::Inch, // Imperial data is in inches
            &params.units,
            params.precision,
        );
        NumberFormatter::format_number_trim_zeros(converted, params.precision)
    };

    // Common annotations for both male and female threads
    let mut multiple_items = vec![
        (SvgText::new(517.0, 194.0 - 5.0, "60°"), text_option_40_0_center_normal),
        (
            SvgText::new(414.5, 624.5 - 5.0, pitch_display),
            text_option_40_0_center_normal,
        ),
        (
            SvgText::new(717.0 - 5.0, 407.5, convert_value(h)),
            text_option_30_90_center_normal,
        ),
        (
            SvgText::new(153.5 + 30.0, 761.0 - 20.0, external_thread),
            text_option_40_0_start_normal,
        ),
        (
            SvgText::new(948.5 - 30.0, 83.0 + 35.0, internal_thread),
            text_option_40_0_end_normal,
        ),
    ];

    // Thread-specific annotations with actual values
    let specific_items = match params.thread_type {
        ThreadType::Male => {
            let major_min_max = format!(
                "ø{}-{}",
                convert_value(thread_data.major_diam_min_m),
                convert_value(thread_data.major_diam_max_m)
            );
            let pitch_min_max = format!(
                "ø{}-{}",
                convert_value(thread_data.pitch_diameter_min_m),
                convert_value(thread_data.pitch_diameter_max_m)
            );
            let minor_min_max = format!(
                "ø{}-{}",
                convert_value(thread_data.unr_minor_diameter_max_m - 0.01),
                convert_value(thread_data.unr_minor_diameter_max_m)
            ); // Approximate range

            vec![
                (
                    SvgText::new(37.0 - 15.0, 555.0, major_min_max),
                    text_option_30_90_center_normal,
                ),
                (
                    SvgText::new(81.0 - 16.0, 590.0, pitch_min_max),
                    text_option_30_90_center_normal,
                ),
                (
                    SvgText::new(131.5 - 17.0, 630.5, minor_min_max),
                    text_option_30_90_center_normal,
                ),
                (
                    SvgText::new(212.0 - 5.0, 372.5, convert_value(h * 17.0 / 24.0)),
                    text_option_30_90_center_normal,
                ),
                (
                    SvgText::new(169.5, 322.5 - 5.0, convert_value(h / 4.0)),
                    text_option_20_0_center_normal,
                ),
                (
                    SvgText::new(356.5, 498. + 15.0, convert_value(h / 8.0)),
                    text_option_30_0_start_normal,
                ),
            ]
        }
        ThreadType::Female => {
            let _major_min_max = format!(
                "ø{}-{}",
                convert_value(thread_data.major_diameter_min_f),
                convert_value(thread_data.major_diameter_min_f + 0.01)
            ); // Approximate range
            let pitch_min_max = format!(
                "ø{}-{}",
                convert_value(thread_data.pitch_diameter_min_f),
                convert_value(thread_data.pitch_diameter_max_f)
            );
            let minor_min_max = format!(
                "ø{}-{}",
                convert_value(thread_data.minor_diameter_min_f),
                convert_value(thread_data.minor_diameter_max_f)
            );

            vec![
                (
                    SvgText::new(917.0 - 17.0, 621.5, minor_min_max),
                    text_option_30_90_center_normal,
                ),
                (
                    SvgText::new(973. - 20.0, 589.5, pitch_min_max),
                    text_option_30_90_center_normal,
                ),
                (
                    SvgText::new(
                        1035. - 5.0,
                        545.5,
                        format!("{} ø{}", min_label, convert_value(thread_data.major_diameter_min_f)),
                    ),
                    text_option_30_90_center_normal,
                ),
                (
                    SvgText::new(820.5 - 5.0, 407.5, convert_value(h * 5.0 / 8.0)),
                    text_option_30_90_center_normal,
                ),
                (
                    SvgText::new(780.0, 309.0 - 5.0, convert_value(h / 4.0)),
                    text_option_20_0_center_normal,
                ),
                (
                    SvgText::new(770.0, 497.5 + 15.0, convert_value(h / 8.0)),
                    text_option_30_0_start_normal,
                ),
            ]
        }
    };
    multiple_items.extend(specific_items);

    svg_content = svg_service.append_text_elements(svg_content, multiple_items, &params.theme);

    let response = svg_service.create_svg_response(svg_content);
    Ok(response)
}
