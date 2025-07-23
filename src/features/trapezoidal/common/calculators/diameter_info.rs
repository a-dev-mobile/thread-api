use crate::shared::enums::{Language, ThreadType, Unit};
use crate::features::trapezoidal::common::models::ModelTrapezoidalDiameterBasic;
use crate::features::trapezoidal::common::models::ModelTrapezoidalDiameterInfo;
use crate::features::trapezoidal::common::models::ModelTrapezoidalTolerance;
use crate::shared::utils::number::NumberFormatter;

use crate::features::trapezoidal::common::enums::TypeTrapezoidalDiameter;

// Структура для локализованных названий измерений
struct DiameterName {
    major: String,
    pitch: String,
    minor: String,
}

impl DiameterName {
    fn new(language: &Language, thread_type: &ThreadType) -> Self {
        match (language, thread_type) {
            (Language::En, ThreadType::Male) => Self {
                major: "Major diameter (d)".to_string(),
                pitch: "Pitch diameter (d2)".to_string(),
                minor: "Minor diameter (d3)".to_string(),
            },
            (Language::Ru, ThreadType::Male) => Self {
                major: "Наружный диаметр (d)".to_string(),
                pitch: "Средний диаметр (d2)".to_string(),
                minor: "Внутренний диаметр (d3)".to_string(),
            },
            (Language::En, ThreadType::Female) => Self {
                major: "Major Diameter (D4)".to_string(),
                pitch: "Pitch diameter (D2)".to_string(),
                minor: "Minor diameter (D1)".to_string(),
            },
            (Language::Ru, ThreadType::Female) => Self {
                major: "Наружный диаметр (D4)".to_string(),
                pitch: "Средний диаметр (D2)".to_string(),
                minor: "Внутренний диаметр (D1)".to_string(),
            },
        }
    }
}

// Форматирование значений с учетом единиц измерения
fn format_measurement_values(
    basic: f64,
    es: f64,
    ei: f64,
    units: &Unit,
    precision: Option<usize>,
) -> (String, String, String, String, String, String) {
    let max = basic + es;
    let min = basic + ei;
    let avg = (max + min) / 2.0;

    let format_value = |value: f64, is_deviation: bool| {
        NumberFormatter::convert_and_round_to_string(
            value,
            &Unit::Mm,
            units,
            precision,
            is_deviation,
        )
    };

    (
        format_value(max, false),   // max
        format_value(es, true),     // es
        format_value(basic, false), // basic
        format_value(avg, false),   // avg
        format_value(ei, true),     // ei
        format_value(min, false),   // min
    )
}

// Создание детального измерения с заполненными значениями
fn create_detailed_measurement(
    name: String,
    enum_type_trapezoidal_diameter: TypeTrapezoidalDiameter,
    basic: f64,
    es: f64,
    ei: f64,
    units: &Unit,
    precision: Option<usize>,
) -> ModelTrapezoidalDiameterInfo {
    let (max, es_str, basic_str, avg, ei_str, min) =
        format_measurement_values(basic, es, ei, units, precision);

    ModelTrapezoidalDiameterInfo {
        type_trapezoidal_diameter: Some(enum_type_trapezoidal_diameter),
        name,
        max,
        es: es_str,
        basic: basic_str,
        avg,
        ei: ei_str,
        min,
    }
}

// Создание измерения для большого диаметра внутренней резьбы
fn create_female_major_measurement(
    name: String,
    basic: f64,
    units: &Unit,
    precision: Option<usize>,
) -> ModelTrapezoidalDiameterInfo {
    ModelTrapezoidalDiameterInfo {
        type_trapezoidal_diameter: Some(TypeTrapezoidalDiameter::Major),
        name,
        max: String::new(),
        es: String::new(),
        basic: String::new(),
        avg: String::new(),
        ei: String::new(),
        min: NumberFormatter::convert_and_round_to_string(
            basic,
            &Unit::Mm,
            units,
            precision,
            false,
        ),
    }
}

// Создание измерений для внешней резьбы
fn create_male_measurements(
    basic_diameters: &ModelTrapezoidalDiameterBasic,
    tolerances: &ModelTrapezoidalTolerance,
    name: &DiameterName,
    units: &Unit,
    precision: Option<usize>,
) -> Vec<ModelTrapezoidalDiameterInfo> {
    vec![
        create_detailed_measurement(
            name.major.clone(),
            TypeTrapezoidalDiameter::Major,
            basic_diameters.d,
            tolerances.es_d,
            tolerances.ei_d,
            units,
            precision,
        ),
        create_detailed_measurement(
            name.pitch.clone(),
            TypeTrapezoidalDiameter::Pitch,
            basic_diameters.d2,
            tolerances.es_d2,
            tolerances.ei_d2,
            units,
            precision,
        ),
        create_detailed_measurement(
            name.minor.clone(),
            TypeTrapezoidalDiameter::Minor,
            basic_diameters.d3,
            tolerances.es_d3,
            tolerances.ei_d3,
            units,
            precision,
        ),
    ]
}

// Создание измерений для внутренней резьбы
fn create_female_measurements(
    basic_diameters: &ModelTrapezoidalDiameterBasic,
    tolerances: &ModelTrapezoidalTolerance,
    nomenclature: &DiameterName,
    units: &Unit,
    precision: Option<usize>,
) -> Vec<ModelTrapezoidalDiameterInfo> {
    vec![
        create_detailed_measurement(
            nomenclature.minor.clone(),
            TypeTrapezoidalDiameter::Minor,
            basic_diameters.d1,
            tolerances.es_d1,
            tolerances.ei_d1,
            units,
            precision,
        ),
        create_detailed_measurement(
            nomenclature.pitch.clone(),
            TypeTrapezoidalDiameter::Pitch,
            basic_diameters.d2,
            tolerances.es_d2,
            tolerances.ei_d2,
            units,
            precision,
        ),
        create_female_major_measurement(
            nomenclature.major.clone(),
            basic_diameters.d4,
            units,
            precision,
        ),
    ]
}

// Основная функция расчета информации о диаметрах
pub fn calculate_diameter_info(
    // params: &RequestTrapezoidalInfo,
    language: Language,
    type_thread: ThreadType,
    units: Unit,
    precision: Option<usize>,
    basic_diameters: &ModelTrapezoidalDiameterBasic,
    tolerances: &ModelTrapezoidalTolerance,
) -> Vec<ModelTrapezoidalDiameterInfo> {
    let name = DiameterName::new(&language, &type_thread);

    match type_thread {
        ThreadType::Male => {
            create_male_measurements(basic_diameters, tolerances, &name, &units, precision)
        }
        ThreadType::Female => {
            create_female_measurements(basic_diameters, tolerances, &name, &units, precision)
        }
    }
}
