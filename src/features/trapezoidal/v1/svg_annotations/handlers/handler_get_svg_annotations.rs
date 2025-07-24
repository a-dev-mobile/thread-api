use axum::body::Body;

use axum::http::Response;
use axum::{extract::Query, Extension};
use sqlx::PgPool;

use crate::features::trapezoidal::v1::svg_annotations::models::RequestSvgAnnotation;
use crate::services::svg::enums::{FontFamily, FontWeight, TextAnchor};
use crate::shared::enums::{Language, ThreadStandard, ThreadType};
use crate::shared::error::AppError;

use crate::services::svg::{SvgService, SvgText, TextOptionsGenerator};

pub async fn handle(
    Extension(pool): Extension<PgPool>,
    Query(params): Query<RequestSvgAnnotation>,
) -> Result<Response<Body>, AppError> {
    let svg_service = SvgService::new("./static/svg");

    let mut svg_content = svg_service
        .load_template(ThreadStandard::Trapezoidal, params.thread_type, params.theme)
        .await?;

    // Create text items to append

    let text_option_40_0_start_normal =
        svg_service.create_custom_text_options(40.0, 0.0, TextAnchor::Start, FontWeight::Normal, FontFamily::Arial);
    let text_option_40_0_center_normal =
        svg_service.create_custom_text_options(40.0, 0.0, TextAnchor::Middle, FontWeight::Normal, FontFamily::Arial);
    let text_option_40_90_center_normal =
        svg_service.create_custom_text_options(40.0, -90.0, TextAnchor::Middle, FontWeight::Normal, FontFamily::Arial);
    let text_option_40_0_end_normal =
        svg_service.create_custom_text_options(40.0, 0.0, TextAnchor::End, FontWeight::Normal, FontFamily::Arial);

    // Get language-specific thread labels
    let (external_thread, internal_thread) = match params.language {
        Language::Ru => ("Наружная резьба", "Внутренняя резьба"),
        Language::En => ("External thread", "Internal thread"),
    };

    // For multiple items:
    let mut multiple_items = vec![
        (SvgText::new(517.0, 194.0 - 5.0, "30°"), text_option_40_0_center_normal),
        (SvgText::new(463.5, 309.0 - 5.0, "ac"), text_option_40_0_center_normal),
        (SvgText::new(516., 462.0 - 5.0, "ac"), text_option_40_0_center_normal),
        (SvgText::new(414.5, 624.5 - 5.0, "P"), text_option_40_0_center_normal),
        (SvgText::new(717.0 - 5.0, 407.5, "H1"), text_option_40_90_center_normal),
        (
            SvgText::new(153.5 + 30.0, 761.0 - 20.0, external_thread),
            text_option_40_0_start_normal,
        ),
        (
            SvgText::new(948.5 - 30.0, 83.0 + 35.0, internal_thread),
            text_option_40_0_end_normal,
        ),
    ];

    let specific_items = match params.thread_type {
        ThreadType::Male => vec![
            (SvgText::new(169.5, 322.5 - 5.0, "R1"), text_option_40_0_center_normal),
            (SvgText::new(356.5, 498. + 15.0, "R2"), text_option_40_0_start_normal),
            (SvgText::new(313.0 - 5.0, 419.0, "h3"), text_option_40_90_center_normal),
            (SvgText::new(37.0 - 5.0, 555.0, "d"), text_option_40_90_center_normal),
            (SvgText::new(81.0 - 5.0, 590.0, "d2"), text_option_40_90_center_normal),
            (SvgText::new(131.5 - 5.0, 630.5, "d3"), text_option_40_90_center_normal),
            (SvgText::new(212.0 - 5.0, 372.5, "z"), text_option_40_90_center_normal),
        ],
        ThreadType::Female => vec![
            (SvgText::new(820.5 - 5.0, 407.5, "H4"), text_option_40_90_center_normal),
            (SvgText::new(917.0 - 5.0, 621.5, "D1"), text_option_40_90_center_normal),
            (SvgText::new(1035. - 5.0, 545.5, "D4"), text_option_40_90_center_normal),
            (SvgText::new(973. - 5.0, 589.5, "D2"), text_option_40_90_center_normal),
            (SvgText::new(780.0, 309.0 - 5.0, "R1"), text_option_40_0_center_normal),
            (SvgText::new(770.0, 497.5 + 15.0, "R2"), text_option_40_0_start_normal),
        ],
    };
    multiple_items.extend(specific_items);

    svg_content = svg_service.append_text_elements(svg_content, multiple_items, &params.theme);

    let response = svg_service.create_svg_response(svg_content);
    Ok(response)
}
