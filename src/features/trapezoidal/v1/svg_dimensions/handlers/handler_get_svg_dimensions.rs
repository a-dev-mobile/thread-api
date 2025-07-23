use axum::body::Body;

use axum::http::{Response, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use axum::{extract::Query, Extension};
use sqlx::PgPool;
use tracing::error;

use crate::services::svg::enums::{FontFamily, FontWeight, TextAnchor};
use crate::shared::enums::{Language, ThreadStandard, ThreadType};
use crate::shared::error::AppError;
use crate::features::trapezoidal::common::calculators::{
    calculate_additional_info, calculate_diameter_info,
};
use crate::features::trapezoidal::common::db::ThreadDataService;
use crate::features::trapezoidal::common::enums::{
    TypeTrapezoidalAdditionalInfo, TypeTrapezoidalDiameter,
};
use crate::features::trapezoidal::v1::svg_dimensions::models::RequestSvgDimension;

use crate::services::svg::{
    SvgService, SvgText,  TextOptionsGenerator,
};

pub async fn handle(
    Extension(pool): Extension<PgPool>,
    Query(params): Query<RequestSvgDimension>,
) -> Result<Response<Body>, AppError> {
    let db_service = ThreadDataService::new(pool);

    // Fetch thread data using the core service
    let thread_data = match db_service
        .fetch_thread_data(
            params.diameter,
            params.pitch,
            params.thread_type,
            &params.tolerance,
        )
        .await
    {
        Ok(data) => data,
        Err(e) => {
            error!("Database query error: {}", e);
            let error_message = if e.to_string().contains("no rows") {
                "No thread specifications found for the given diameter and pitch"
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

    let diameter_info = calculate_diameter_info(
        params.language,
        params.thread_type,
        params.units,
        params.precision,
        &thread_data.basic_diameters,
        &thread_data.tolerances,
    );
    let additional_info = calculate_additional_info(
        params.language,
        params.thread_type,
        params.units,
        params.precision,
        &thread_data,
    );
    // Extract specific diameter information
    let get_diameter = |enum_type: TypeTrapezoidalDiameter| {
        diameter_info
            .iter()
            .find(|d| d.type_trapezoidal_diameter == Some(enum_type))
            .unwrap_or_else(|| panic!("{:?} diameter info not found", enum_type))
    };

    let get_addition_info = |enum_type: TypeTrapezoidalAdditionalInfo| {
        additional_info
            .iter()
            .find(|d| d.type_trapezoidal_additional_info == Some(enum_type))
            .unwrap_or_else(|| panic!("{:?} addition info not found", enum_type))
    };

    let major_diameter = get_diameter(TypeTrapezoidalDiameter::Major);
    let pitch_diameter = get_diameter(TypeTrapezoidalDiameter::Pitch);
    let minor_diameter = get_diameter(TypeTrapezoidalDiameter::Minor);
    let ac = get_addition_info(TypeTrapezoidalAdditionalInfo::ac);
    let h1 = get_addition_info(TypeTrapezoidalAdditionalInfo::H1);
    let h4_h3 = get_addition_info(TypeTrapezoidalAdditionalInfo::H4_h3);
    let r1 = get_addition_info(TypeTrapezoidalAdditionalInfo::r1_max);
    let r2 = get_addition_info(TypeTrapezoidalAdditionalInfo::r2_max);
    let z = get_addition_info(TypeTrapezoidalAdditionalInfo::z);

    let svg_service = SvgService::new("./static/svg");

    let mut svg_content = svg_service
        .load_template(
            ThreadStandard::Trapezoidal,
            params.thread_type,
            params.theme,
        )
        .await?;
    //
    //
    //
    //
    //
    //
    //
    //
    let text_options_default = svg_service.create_custom_text_options(
        40.0,
        0.0,
        TextAnchor::Start,
        FontWeight::Normal,
        FontFamily::Arial,
    );
    let text_option_30_90_center_normal = svg_service.create_custom_text_options(
        30.0,
        -90.0,
        TextAnchor::Middle,
        FontWeight::Normal,
        FontFamily::Arial,
    );
    let text_option_20_0_center_normal = svg_service.create_custom_text_options(
        20.0,
        0.0,
        TextAnchor::Middle,
        FontWeight::Normal,
        FontFamily::Arial,
    );
    let text_option_20_90_center_normal = svg_service.create_custom_text_options(
        20.0,
        -90.0,
        TextAnchor::Middle,
        FontWeight::Normal,
        FontFamily::Arial,
    );
    let text_option_17_90_center_normal = svg_service.create_custom_text_options(
        17.0,
        -90.0,
        TextAnchor::Middle,
        FontWeight::Normal,
        FontFamily::Arial,
    );
    let text_option_30_0_start_normal = svg_service.create_custom_text_options(
        30.0,
        0.0,
        TextAnchor::Start,
        FontWeight::Normal,
        FontFamily::Arial,
    );
    let text_option_30_0_center_normal = svg_service.create_custom_text_options(
        30.0,
        0.0,
        TextAnchor::Middle,
        FontWeight::Normal,
        FontFamily::Arial,
    );
    let text_option_40_0_start_normal = svg_service.create_custom_text_options(
        40.0,
        0.0,
        TextAnchor::Start,
        FontWeight::Normal,
        FontFamily::Arial,
    );
    let text_option_40_0_center_normal = svg_service.create_custom_text_options(
        40.0,
        0.0,
        TextAnchor::Middle,
        FontWeight::Normal,
        FontFamily::Arial,
    );
    let text_option_40_0_end_normal = svg_service.create_custom_text_options(
        40.0,
        0.0,
        TextAnchor::End,
        FontWeight::Normal,
        FontFamily::Arial,
    );
    let text_options_diameters_avg = svg_service.create_custom_text_options(
        12.0,
        -90.0,
        TextAnchor::Middle,
        FontWeight::Normal,
        FontFamily::Arial,
    );
    let text_option_40_0_end_bold = svg_service.text_option_40_0_end_bold();

    // Get language-specific thread labels
    let (external_thread, internal_thread, avg_label, min_label) = match params.language {
        Language::Ru => ("Наружная резьба", "Внутренняя резьба", "сред.", "мин."),
        Language::En => ("External thread", "Internal thread", "avg.", "min."),
    };
    let format_min_max = |min: &str, max: &str| format!("ø{}-{}", min, max);
    let format_avg = |label: &str, value: &str| format!("({} ø{})", label, value);

    let pitch_min_max = format_min_max(&pitch_diameter.min, &pitch_diameter.max);
    let pitch_avg = format_avg(avg_label, &pitch_diameter.avg);

    let major_min_max = format_min_max(&major_diameter.min, &major_diameter.max);
    let major_avg = format_avg(avg_label, &major_diameter.avg);

    let minor_min_max = format_min_max(&minor_diameter.min, &minor_diameter.max);
    let minor_avg = format_avg(avg_label, &minor_diameter.avg);

    // For multiple items:
    let mut multiple_items = vec![
        (
            SvgText::new(517.0, 194.0 - 5.0, "30°"),
            text_option_40_0_center_normal,
        ),
        (
            SvgText::new(463.5 - 8., 309.0 - 5.0, ac.value.clone()),
            text_option_30_0_start_normal,
        ),
        (
            SvgText::new(516., 462.0 - 5.0, ac.value.clone()),
            text_option_20_0_center_normal,
        ),
        (
            SvgText::new(414.5, 624.5 - 5.0, format!("{}", &params.pitch)),
            text_option_40_0_center_normal,
        ),
        (
            SvgText::new(717.0 - 5.0, 407.5, h1.value.clone()),
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

    let specific_items = match params.thread_type {
        ThreadType::Male => vec![
            (
                SvgText::new(169.5, 322.5 - 5.0, r1.value.clone()),
                text_option_30_0_center_normal,
            ),
            (
                SvgText::new(356.5, 498. + 15.0, r2.value.clone()),
                text_option_30_0_start_normal,
            ),
            (
                SvgText::new(313.0 - 5.0, 419.0, h4_h3.value.clone()),
                text_option_30_90_center_normal,
            ),
            //
            (
                SvgText::new(37.0 - 15.0, 555.0, major_min_max),
                text_option_30_90_center_normal,
            ),
            (
                SvgText::new(37.0 - 5.0, 555.0, major_avg),
                text_options_diameters_avg,
            ),
            //
            (
                SvgText::new(81.0 - 16.0, 590.0, pitch_min_max),
                text_option_30_90_center_normal,
            ),
            (
                SvgText::new(81.0 - 5.0, 590.0, pitch_avg),
                text_options_diameters_avg,
            ),
            // minor
            (
                SvgText::new(131.5 - 17.0, 630.5, minor_min_max),
                text_option_30_90_center_normal,
            ),
            (
                SvgText::new(131.5 - 5.0, 630.5, minor_avg),
                text_options_diameters_avg,
            ),
            (
                SvgText::new(212.0 - 5.0, 372.5, z.value.clone()),
                text_option_17_90_center_normal,
            ),
        ],
        ThreadType::Female => vec![
            (
                SvgText::new(820.5 - 5.0, 407.5, h4_h3.value.clone()),
                text_option_30_90_center_normal,
            ),
            (
                SvgText::new(917.0 - 17.0, 621.5, minor_min_max),
                text_option_30_90_center_normal,
            ),
            (
                SvgText::new(917.0 - 5.0, 621.5, minor_avg),
                text_options_diameters_avg,
            ),
            (
                SvgText::new(
                    1035. - 5.0,
                    545.5,
                    format!("{} ø{}", min_label, &major_diameter.min),
                ),
                text_option_30_90_center_normal,
            ),
            (
                SvgText::new(973. - 20.0, 589.5, pitch_min_max),
                text_option_30_90_center_normal,
            ),
            (
                SvgText::new(973. - 5.0, 589.5, pitch_avg),
                text_options_diameters_avg,
            ),
            (
                SvgText::new(780.0, 309.0 - 5.0, r1.value.clone()),
                text_option_30_0_center_normal,
            ),
            (
                SvgText::new(770.0, 497.5 + 15.0, r2.value.clone()),
                text_option_30_0_center_normal,
            ),
        ],
    };
    multiple_items.extend(specific_items);

    svg_content = svg_service.append_text_elements(svg_content, multiple_items, &params.theme);

    let response = svg_service.create_svg_response(svg_content);
    Ok(response)
}
