use crate::shared::enums::{Language, ThreadType, Unit};

use crate::features::trapezoidal::common::models::model_trapezoidal_additional_info::ModelTrapezoidalAdditionalInfo;
use crate::shared::utils::number::NumberFormatter;

pub fn calculate_main_info(
    diameter: i32,
    pitch: f64,
    thread_depth: f64,
    tolerance: String,
    type_thread: ThreadType,
    language: Language,
    units: Unit,
    precision: Option<usize>,
) -> Vec<ModelTrapezoidalAdditionalInfo> {
    match language {
        Language::En => vec![
            create_info_item("Thread type", thread_type_text(type_thread, language), None),
            create_info_item(
                "Diameter (nominal)",
                format_value(diameter as f64, units, precision),
                None,
            ),
            create_info_item("Thread pitch", format_value(pitch, units, precision), None),
            create_info_item("Thread tolerance", tolerance, None),
            create_info_item("Thread depth", format_value(thread_depth, units, precision), None),
        ],
        Language::Ru => vec![
            create_info_item("Тип резьбы", thread_type_text(type_thread, language), None),
            create_info_item(
                "Диаметр (номинальный)",
                format_value(diameter as f64, units, precision),
                None,
            ),
            create_info_item("Шаг резьбы", format_value(pitch, units, precision), None),
            create_info_item("Допуск резьбы", tolerance, None),
            create_info_item("Глубина резьбы", format_value(thread_depth, units, precision), None),
        ],
    }
}

fn format_value(value: f64, units: Unit, precision: Option<usize>) -> String {
    NumberFormatter::convert_and_round_to_string(value, &Unit::Mm, &units, precision, false)
}

fn create_info_item(name: &str, value: String, description: Option<&str>) -> ModelTrapezoidalAdditionalInfo {
    ModelTrapezoidalAdditionalInfo {
        type_trapezoidal_additional_info: None,
        name: name.to_string(),
        value,
        description: description.map(String::from),
    }
}

fn thread_type_text(thread_type: ThreadType, language: Language) -> String {
    match (thread_type, language) {
        (ThreadType::Male, Language::En) => "External Thread".to_string(),
        (ThreadType::Female, Language::En) => "Internal Thread".to_string(),
        (ThreadType::Male, Language::Ru) => "Наружная резьба".to_string(),
        (ThreadType::Female, Language::Ru) => "Внутренняя резьба".to_string(),
    }
}

pub fn get_thread_info(
    language: Language,
    thread_type: ThreadType,
    diameter: i32,
    pitch: f64,
    tolerance: &str,
) -> (String, String) {
    match (language, thread_type) {
        (Language::En, ThreadType::Male) => (
            "Tr - Single-start external trapezoidal thread".to_string(),
            format!("Tr {} x {} - {}", diameter, pitch, tolerance),
        ),
        (Language::En, ThreadType::Female) => (
            "Tr - Single-start internal trapezoidal thread".to_string(),
            format!("Tr {} x {} - {}", diameter, pitch, tolerance),
        ),
        (Language::Ru, ThreadType::Male) => (
            "Tr - Трапецеидальная однозаходная наружная резьба".to_string(),
            format!("Tr {} x {} - {}", diameter, pitch, tolerance),
        ),
        (Language::Ru, ThreadType::Female) => (
            "Tr - Трапецеидальная однозаходная внутренняя резьба".to_string(),
            format!("Tr {} x {} - {}", diameter, pitch, tolerance),
        ),
    }
}
