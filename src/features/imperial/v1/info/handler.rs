use axum::{
    extract::{Extension, Query},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use sqlx::PgPool;
use crate::log_error;

use crate::{
    analytics::db::handle_thread_analytics,shared::enums::{Language, ThreadType, Unit}, shared::utils::number::NumberFormatter, features::imperial::v1::info::{
        additional_info::additional_thread_info,
        models::{DbModel, ImperialInfoResponse, RequestParams},
    }
};

/// Обработчик запроса
pub async fn handle(
    Extension(pool): Extension<PgPool>,
    Query(params): Query<RequestParams>,
) -> impl IntoResponse {
    let is_male = matches!(params.type_, ThreadType::Male);

    // Build query based on thread type
    let query = if is_male {
        "SELECT * FROM imperial.main WHERE diameter = $1 AND tpi = $2 AND class_m = $3"
    } else {
        "SELECT * FROM imperial.main WHERE diameter = $1 AND tpi = $2 AND class_f = $3"
    };

    match sqlx::query_as::<_, DbModel>(query)
        .bind(&params.diameter)
        .bind(params.tpi)
        .bind(&params.series)
        .fetch_one(&pool)
        .await
    {
        Ok(record) => {
            let is_male = matches!(params.type_, ThreadType::Male);
            let record_clone = record.clone();
            let params_clone = params.clone();

            // Generate designation1 early for analytics
            let class = if is_male {
                &record.class_m
            } else {
                &record.class_f
            };
            let designation = format!(
                "{} - {} {} - {}",
                record.diameter, record.tpi, record.series_designation, class
            );

            // Clone pool and designation for background task
            let pool_clone = pool.clone();
            let designation_clone = designation.clone();

            // Spawn background task for analytics
            tokio::spawn(async move {
                handle_thread_analytics(pool_clone, designation_clone).await;
            });

            let tpi = record.tpi;
            let pitch = 1.0 / tpi;
            let d_basic = record.diameter_2;
            let le: f64 = 9.0 * pitch;
            let h = (3f64.sqrt() / 2.0) * pitch;
            // Get the appropriate class based on thread type
            let class = if is_male {
                &record.class_m
            } else {
                &record.class_f
            };

            let description = generate_description(&params.type_, &params.language);
            let (designation1, designation2) = generate_designation(&record, &params);

            // Map the appropriate values based on thread type
            let major_diam_max = if is_male {
                record.major_diam_max_m
            } else {
                d_basic // For female threads, max is basic diameter
            };

            let major_diam_min = if is_male {
                record.major_diam_min_m
            } else {
                record.major_diameter_min_f
            };

            let pitch_diameter_max = if is_male {
                record.pitch_diameter_max_m
            } else {
                record.pitch_diameter_max_f
            };

            let pitch_diameter_min = if is_male {
                record.pitch_diameter_min_m
            } else {
                record.pitch_diameter_min_f
            };

            let pitch_diameter_tolerance = (pitch_diameter_max - pitch_diameter_min).abs();

            let minor_diameter_max = if is_male {
                let (_, d1_max) = calculate_minor_diam_male(major_diam_max, 1.0 / tpi);
                d1_max
            } else {
                record.minor_diameter_max_f
            };

            // Calculate basic diameters
            let (major_diameter_basic, pitch_diameter_basic, minor_diameter_basic) =
                calculate_basic_diameters(d_basic, h);

            let minor_diameter_min = if is_male {
                minor_diameter_basic
            } else {
                record.minor_diameter_min_f
            };

            let unr_minor_diameter_max = if is_male {
                Some(record.unr_minor_diameter_max_m)
            } else {
                None
            };
            let t_d2 = calculate_td2(d_basic, le, pitch, class);
            let t_d = calculate_td(pitch);
            let allowance = calculate_allowance(t_d2, 0.3);

            let units = params.units;
            let precision = params.precision;

            // Расчёт отклонений для major diameter
            let major_diam_es = major_diam_max - major_diameter_basic;
            let major_diam_ei = major_diam_min - major_diameter_basic;

            // Расчёт отклонений для pitch diameter (значения не Option, поэтому можно сразу)
            let pitch_diameter_es = pitch_diameter_max - pitch_diameter_basic;
            let pitch_diameter_ei = pitch_diameter_min - pitch_diameter_basic;

            // Update the minor diameter related calculations
            let minor_diam_es = minor_diameter_max - minor_diameter_basic;
            let minor_diam_ei = minor_diameter_min - minor_diameter_basic;

            let response = ImperialInfoResponse {
                id: record.id,
                fractional_diameter: record.diameter,
                description,
                designation1,
                designation2,
                decimal_diameter: NumberFormatter::convert_and_round(
                    d_basic,
                    &Unit::Inch,
                    &units,
                    params.precision,
                ),
                tpi: tpi as i32,
                pitch: NumberFormatter::convert_and_round(
                    pitch,
                    &Unit::Inch,
                    &units,
                    params.precision,
                ),
                series_designation: record.series_designation,
                series: class.clone(),
                type_: params.type_,
                t_d2: NumberFormatter::convert_and_round(
                    t_d2,
                    &Unit::Inch,
                    &units,
                    params.precision,
                ),
                t_d: NumberFormatter::convert_and_round(t_d, &Unit::Inch, &units, params.precision),
                allowance: Some(NumberFormatter::convert_and_round(
                    allowance,
                    &Unit::Inch,
                    &units,
                    params.precision,
                )),
                major_diam_max: NumberFormatter::convert_and_round(
                    major_diam_max,
                    &Unit::Inch,
                    &units,
                    params.precision,
                ),
                major_diameter_basic: NumberFormatter::convert_and_round(
                    major_diameter_basic,
                    &Unit::Inch,
                    &units,
                    params.precision,
                ),
                major_diam_min: NumberFormatter::convert_and_round(
                    major_diam_min,
                    &Unit::Inch,
                    &units,
                    params.precision,
                ),
                pitch_diameter_max: NumberFormatter::convert_and_round(
                    pitch_diameter_max,
                    &Unit::Inch,
                    &units,
                    params.precision,
                ),
                pitch_diameter_basic: NumberFormatter::convert_and_round(
                    pitch_diameter_basic,
                    &Unit::Inch,
                    &units,
                    params.precision,
                ),
                pitch_diameter_min: NumberFormatter::convert_and_round(
                    pitch_diameter_min,
                    &Unit::Inch,
                    &units,
                    params.precision,
                ),
                minor_diameter_max: NumberFormatter::convert_and_round(
                    minor_diameter_max,
                    &Unit::Inch,
                    &units,
                    params.precision,
                ),
                minor_diameter_basic: NumberFormatter::convert_and_round(
                    minor_diameter_basic,
                    &Unit::Inch,
                    &units,
                    params.precision,
                ),
                minor_diameter_min: NumberFormatter::convert_and_round(
                    minor_diameter_min,
                    &Unit::Inch,
                    &units,
                    params.precision,
                ),
                unr_minor_diameter_max: unr_minor_diameter_max.map(|val| {
                    NumberFormatter::convert_and_round(val, &Unit::Inch, &units, params.precision)
                }),
                pitch_diameter_tolerance: NumberFormatter::convert_and_round(
                    pitch_diameter_tolerance,
                    &Unit::Inch,
                    &units,
                    params.precision,
                ),
                major_diam_min2: record
                    .major_diam_min2_m // Fix field name
                    .map(|val| {
                        NumberFormatter::convert_and_round(
                            val,
                            &Unit::Inch,
                            &units,
                            params.precision,
                        )
                    }),
                units: params.units,
                h: NumberFormatter::convert_and_round(h, &Unit::Inch, &units, params.precision),
                thread_depth: NumberFormatter::convert_and_round(
                    0.625 * h,
                    &Unit::Inch,
                    &units,
                    params.precision,
                ),

                // Calculate and add average diameters
                major_diameter_avg: NumberFormatter::convert_and_round(
                    (major_diam_max + major_diam_min) / 2.0,
                    &Unit::Inch,
                    &units,
                    params.precision,
                ),
                pitch_diameter_avg: NumberFormatter::convert_and_round(
                    (pitch_diameter_max + pitch_diameter_min) / 2.0,
                    &Unit::Inch,
                    &units,
                    params.precision,
                ),
                minor_diameter_avg: match (minor_diameter_max, minor_diameter_min) {
                    (max, min) => NumberFormatter::convert_and_round(
                        (max + min) / 2.0,
                        &Unit::Inch,
                        &units,
                        params.precision,
                    ),
                },

                // Новые поля допусков (отклонений) с форматированием
                major_diam_es: NumberFormatter::convert_and_round(
                    major_diam_es,
                    &Unit::Inch,
                    &units,
                    params.precision,
                ),
                major_diam_ei: NumberFormatter::convert_and_round(
                    major_diam_ei,
                    &Unit::Inch,
                    &units,
                    params.precision,
                ),
                pitch_diameter_es: NumberFormatter::convert_and_round(
                    pitch_diameter_es,
                    &Unit::Inch,
                    &units,
                    params.precision,
                ),
                pitch_diameter_ei: NumberFormatter::convert_and_round(
                    pitch_diameter_ei,
                    &Unit::Inch,
                    &units,
                    params.precision,
                ),
                minor_diam_es: NumberFormatter::convert_and_round(
                    minor_diam_es,
                    &Unit::Inch,
                    &units,
                    params.precision,
                ),
                minor_diam_ei: NumberFormatter::convert_and_round(
                    minor_diam_ei,
                    &Unit::Inch,
                    &units,
                    params.precision,
                ),
                additional_info: additional_thread_info(&params_clone, &record_clone),
            };

            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => {
            log_error!("Database query error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Thread with diameter: {}, TPI: {}, class: {} not found",
                        params.diameter, params.tpi, params.series)
                })),
            )
                .into_response()
        }
    }
}

/// Вспомогательная функция для генерации описания резьбы
fn generate_description(thread_type: &ThreadType, language: &Language) -> String {
    match language {
        Language::Ru => format!(
            "Унифицированная цилиндрическая {} резьба",
            match thread_type {
                ThreadType::Male => "наружная",
                ThreadType::Female => "внутренняя",
            }
        ),
        Language::En => format!(
            "Unified cylindrical {} thread",
            match thread_type {
                ThreadType::Male => "external",
                ThreadType::Female => "internal",
            }
        ),
    }
}

/// Вспомогательная функция для генерации описания резьбы

fn generate_designation(db_model: &DbModel, params: &RequestParams) -> (String, String) {
    let class = if matches!(params.type_, ThreadType::Male) {
        &db_model.class_m
    } else {
        &db_model.class_f
    };

    let fractional = format!(
        "{} - {} {} - {}",
        db_model.diameter, db_model.tpi, db_model.series_designation, class
    );

    let decimal = format!(
        "{} - {} {} - {}",
        db_model.diameter_2, db_model.tpi, db_model.series_designation, class
    );

    (fractional, decimal)
}

// Вычисление вспомогательных параметров резьбы
pub fn calculate_td2(d: f64, le: f64, p: f64, class: &str) -> f64 {
    // Calculate each component separately for logging
    let d_term = 0.0015 * d.powf(1.0 / 3.0);
    let le_term = 0.0015 * le.sqrt();
    let p_term = 0.015 * p.powf(2.0 / 3.0);

    // Base calculation for class 2A
    // https://www.machiningdoctor.com/charts/unified-inch-threads-charts/
    let t = d_term + le_term + p_term;

    let result = match class {
        "1A" => 1.500 * t,
        "2A" => t,
        "3A" => 0.750 * t,
        "3B" => 0.975 * t,
        "2B" => 1.300 * t,
        "1B" => 1.950 * t,
        _ => 0.0,
    };

    println!("Final TD2 for class {} = {}", class, result);
    result
}

fn calculate_td(pitch: f64) -> f64 {
    0.060 * (pitch.powi(2)).cbrt()
}

fn calculate_allowance(t_d2: f64, p1: f64) -> f64 {
    // allowance = p1 * T_d2(2A)
    p1 * t_d2
}

fn calculate_major_diam_male(allowance: f64, d_basic: f64, td: f64) -> (f64, f64) {
    // Вычисление Td₂(2A) по формуле
    let max = d_basic - allowance;
    let min = max - td;

    (max, min)
}

fn calculate_major_diam_female(d_basic: f64) -> f64 {
    d_basic
}
// https://fpg-co.com/Standards/ASME%20B1.1%202008.pdf стр 88
fn calculate_pitch_diam_male(d_max: f64, pitch: f64, d_basic: f64, le: f64) -> (f64, f64) {
    let t_d2 = 0.0015 * d_basic.cbrt() + 0.0015 * le.sqrt() + 0.015 * (pitch.powf(2.0 / 3.0));
    let d2_max = d_max - 0.649519 * pitch;
    let d2_min = d2_max - t_d2;

    (d2_max, d2_min)
}
// https://fpg-co.com/Standards/ASME%20B1.1%202008.pdf стр 91
fn calculate_pitch_diam_female(d_basic: f64, pitch: f64, t_d2: f64) -> (f64, f64) {
    // D2_min
    let pitch_diam_min = d_basic - 0.64951905 * pitch;
    // D2_max
    let pitch_diam_max = pitch_diam_min + t_d2;

    (pitch_diam_max, pitch_diam_min)
}
// https://fpg-co.com/Standards/ASME%20B1.1%202008.pdf стр 89
fn calculate_minor_diam_male(major_diam_max_male: f64, pitch: f64) -> (f64, f64) {
    // Maximum external UNR minor diameter
    let d3_max = major_diam_max_male - 1.19078493 * pitch;
    // Maximum external UN minor diameter
    let d1_max = major_diam_max_male - 1.08253175 * pitch;
    (d3_max, d1_max)
}

// https://fpg-co.com/Standards/ASME%20B1.1%202008.pdf стр 89
fn calculate_minor_diam_female(d_basic: f64, pitch: f64) -> (f64, f64) {
    // D1_min
    let minor_diam_min = d_basic - 1.08253175 * pitch;
    // D1_max
    let minor_diam_max = minor_diam_min + (0.25 * pitch - 0.4 * (pitch.powi(2)));
    (minor_diam_max, minor_diam_min)
}

// Add these functions after existing calculation functions
fn calculate_basic_diameters(d_basic: f64, h: f64) -> (f64, f64, f64) {
    // Major diameter basic is same as d_basic
    let major_diameter_basic = d_basic;

    // Pitch diameter basic = d - (2 × 3/8)H
    let pitch_diameter_basic = d_basic - (2.0 * 0.375 * h);

    // Minor diameter basic = d - (2 × 5/8)H
    let minor_diameter_basic = d_basic - (2.0 * 0.625 * h);

    (
        major_diameter_basic,
        pitch_diameter_basic,
        minor_diameter_basic,
    )
}

pub fn unified_thread_allowance(d: f64, p: f64, le: f64, class: &str) -> Result<f64, String> {
    // Handle thread class logic
    match class {
        "1A" | "2A" => {
            // Use the provided LE or calculate the default value (5 * P)

            // Calculate the allowance according to the formula
            let es = 0.3 * (0.0015 * d.cbrt() + 0.0015 * le.sqrt() + 0.015 * (p.powi(2)).cbrt());

            // Round the result to 6 decimal places
            Ok((es * 1_000_000.0).round() / 1_000_000.0)
        }
        "3A" => Ok(0.0), // Return 0 for class 3A
        _ => Err("Invalid thread class. Calculation failed.".to_string()), // Return an error for other classes
    }
}
