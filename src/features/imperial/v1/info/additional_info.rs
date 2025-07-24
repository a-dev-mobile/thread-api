use crate::shared::{
    enums::{Language, ThreadType, Unit},
    utils::number::NumberFormatter,
};

use super::models::{DbModel, RequestParams, ThreadInfoItem};

/// Calculates H and related thread parameters
fn calculate_h(pitch: f64) -> (f64, f64, f64, f64, f64) {
    let h = (3f64.sqrt() / 2.0) * pitch;
    let h_9_16 = (9.0 / 16.0) * h;
    let h_3_8 = (3.0 / 8.0) * h;
    let h_5_16 = (5.0 / 16.0) * h;
    let h_1_8 = h / 8.0;

    (h, h_9_16, h_3_8, h_5_16, h_1_8)
}

/// Generates thread info items with descriptions
pub fn additional_thread_info(params: &RequestParams, db: &DbModel) -> Vec<ThreadInfoItem> {
    let tpi = db.tpi;
    let pitch = 1.0 / tpi;

    let (h, h_9_16, h_3_8, h_5_16, h_1_8) = calculate_h(pitch);

    let language = &params.language;
    let units = &params.units;
    let precision = params.precision;

    match language {
        Language::En => {
            let mut items = Vec::new();

            // Add allowance only for male threads and when allowance is not zero
            if matches!(params.type_, ThreadType::Male) && db.allowance != 0.0 {
                items.push(ThreadInfoItem {
                    name: "Allowance".to_string(),
                    value: NumberFormatter::convert_and_round(db.allowance, &Unit::Inch, units, precision).to_string(),
                    description: None,
                });
            }

            items.extend(vec![
                ThreadInfoItem {
                    name: "Height of fundamental triangle (H)".to_string(),
                    value: NumberFormatter::convert_and_round(h, &Unit::Inch, units, precision).to_string(),
                    description: None,
                },
                ThreadInfoItem {
                    name: "Truncation at crest (H/8)".to_string(),
                    value: NumberFormatter::convert_and_round(h_1_8, &Unit::Inch, units, precision).to_string(),
                    description: None,
                },
                ThreadInfoItem {
                    name: "Basic truncation at root (internal thread) (5H/16)".to_string(),
                    value: NumberFormatter::convert_and_round(h_5_16, &Unit::Inch, units, precision).to_string(),
                    description: None,
                },
                ThreadInfoItem {
                    name: "Basic thread depth (3H/8)".to_string(),
                    value: NumberFormatter::convert_and_round(h_3_8, &Unit::Inch, units, precision).to_string(),
                    description: None,
                },
                ThreadInfoItem {
                    name: "Maximum material at root (external thread) (9H/16)".to_string(),
                    value: NumberFormatter::convert_and_round(h_9_16, &Unit::Inch, units, precision).to_string(),
                    description: None,
                },
            ]);
            items
        }
        Language::Ru => {
            let mut items = Vec::new();

            // Add allowance only for male threads and when allowance is not zero
            if matches!(params.type_, ThreadType::Male) && db.allowance != 0.0 {
                items.push(ThreadInfoItem {
                    name: "Допуск".to_string(),
                    value: NumberFormatter::convert_and_round(db.allowance, &Unit::Inch, units, precision).to_string(),
                    description: None,
                });
            }

            items.extend(vec![
                ThreadInfoItem {
                    name: "Высота исходного треугольника резьбы (H)".to_string(),
                    value: NumberFormatter::convert_and_round(h, &Unit::Inch, units, precision).to_string(),
                    description: None,
                },
                ThreadInfoItem {
                    name: "Срез по вершине резьбы (H/8)".to_string(),
                    value: NumberFormatter::convert_and_round(h_1_8, &Unit::Inch, units, precision).to_string(),
                    description: None,
                },
                ThreadInfoItem {
                    name: "Базовый срез у впадины (внутренняя резьба) (5H/16)".to_string(),
                    value: NumberFormatter::convert_and_round(h_5_16, &Unit::Inch, units, precision).to_string(),
                    description: None,
                },
                ThreadInfoItem {
                    name: "Базовая глубина резьбы (3H/8)".to_string(),
                    value: NumberFormatter::convert_and_round(h_3_8, &Unit::Inch, units, precision).to_string(),
                    description: None,
                },
                ThreadInfoItem {
                    name: "Максимальный материал у впадины (наружная резьба) (9H/16)".to_string(),
                    value: NumberFormatter::convert_and_round(h_9_16, &Unit::Inch, units, precision).to_string(),
                    description: None,
                },
            ]);
            items
        }
    }
}
