use crate::shared::enums::{Language, ThreadType, Unit};

use crate::features::trapezoidal::common::db::ThreadData;
use crate::features::trapezoidal::common::enums::TypeTrapezoidalAdditionalInfo;
use crate::features::trapezoidal::common::models::ModelTrapezoidalAdditionalInfo;
use crate::shared::utils::number::NumberFormatter;

pub fn calculate_additional_info(
    language: Language,
    type_thread: ThreadType,
    units: Unit,
    precision: Option<usize>,
    thread_data: &ThreadData,
) -> Vec<ModelTrapezoidalAdditionalInfo> {
    let label_max = match language {
        Language::En => "max",
        Language::Ru => "макс",
    };

    vec![
        // ac
        ModelTrapezoidalAdditionalInfo {
            type_trapezoidal_additional_info: Some(TypeTrapezoidalAdditionalInfo::ac),
            name: match language {
                Language::En => "Clearance on the crest (ac)",
                Language::Ru => "Зазор по вершине резьбы (ac)",
            }
            .to_string(),
            value: NumberFormatter::convert_and_round(
                thread_data.other_dimensions.a_c,
                &Unit::Mm,
                &units,
                precision,
            )
            .to_string(),
            description: None,
        },
        // h3_H4
        ModelTrapezoidalAdditionalInfo {
            type_trapezoidal_additional_info: Some(TypeTrapezoidalAdditionalInfo::H4_h3),
            name: match type_thread {
                ThreadType::Male => match language {
                    Language::En => "Height of external threads (h3)",
                    Language::Ru => "Высота профиля резьбы (h3)",
                },
                ThreadType::Female => match language {
                    Language::En => "Height of internal threads (H4)",
                    Language::Ru => "Высота профиля резьбы (H4)",
                },
            }
            .to_string(),

            value: NumberFormatter::convert_and_round(
                thread_data.other_dimensions.h4_h3,
                &Unit::Mm,
                &units,
                precision,
            )
            .to_string(),
            description: None,
        },
        // H1
        ModelTrapezoidalAdditionalInfo {
            type_trapezoidal_additional_info: Some(TypeTrapezoidalAdditionalInfo::H1),
            name: match language {
                Language::En => "Height of the overlapping (H1)",
                Language::Ru => "Рабочая высота профиля резьбы (H1)",
            }
            .to_string(),
            value: NumberFormatter::convert_and_round(
                thread_data.other_dimensions.h1,
                &Unit::Mm,
                &units,
                precision,
            )
            .to_string(),
            description: None,
        },
        // r1_max
        ModelTrapezoidalAdditionalInfo {
            type_trapezoidal_additional_info: Some(TypeTrapezoidalAdditionalInfo::r1_max),
            name: match language {
                Language::En => "Radius at the top of thread (R1max)",
                Language::Ru => "Радиус вершины резьбы (R1max)",
            }
            .to_string(),
            value: NumberFormatter::convert_and_round(
                thread_data.other_dimensions.r1_max,
                &Unit::Mm,
                &units,
                precision,
            )
            .to_string(),
            description: None,
        },
        // r2_max
        ModelTrapezoidalAdditionalInfo {
            type_trapezoidal_additional_info: Some(TypeTrapezoidalAdditionalInfo::r2_max),
            name: match language {
                Language::En => "Radius at the bottom of thread (R2max)",
                Language::Ru => "Радиус впадины резьбы (R2max)",
            }
            .to_string(),
            value: NumberFormatter::convert_and_round(
                thread_data.other_dimensions.r2_max,
                &Unit::Mm,
                &units,
                precision,
            )
            .to_string(),
            description: None,
        },
        // z
        ModelTrapezoidalAdditionalInfo {
            type_trapezoidal_additional_info: Some(TypeTrapezoidalAdditionalInfo::z),
            name: match language {
                Language::En => "1/2 working height of thread profile (z)",
                Language::Ru => "1/2 рабочей высоты профиля резьбы (z)",
            }
            .to_string(),
            value: NumberFormatter::convert_and_round(
                thread_data.other_dimensions.z,
                &Unit::Mm,
                &units,
                precision,
            )
            .to_string(),
            description: None,
        },
    ]
}
