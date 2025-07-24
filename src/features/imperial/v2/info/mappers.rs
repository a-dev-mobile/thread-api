use crate::{
    features::imperial::v2::info::models::response::ResponseV2ImperialInfo,
    shared::enums::{Language, ThreadType, Unit},
    shared::utils::number::NumberFormatter,
};

use super::models::{
    db::ModelV2ImperialDB,
    request::RequestV2ImperialInfo,
    response::{ModelImperialAdditionalInfo, ModelImperialDiameterInfo},
};

pub struct ImperialInfoMapper;

impl ImperialInfoMapper {
    pub fn from_data(db: ModelV2ImperialDB, request: &RequestV2ImperialInfo) -> ResponseV2ImperialInfo {
        let is_male = matches!(request.type_, ThreadType::Male);

        ResponseV2ImperialInfo {
            designation1: Self::generate_designation1(&db, &request.type_),
            designation2: Self::generate_designation2(&db, &request.type_),
            description: Self::format_description(is_male, &request.language),
            unit: Self::format_unit(&request.language, &request.units),
            main_info: Self::map_main_info(is_male, &db, request),
            diameter_info: Self::map_diameter_info(&db, request),
            additional_info: Self::map_additional_info(&db, request),
        }
    }

    pub fn generate_designation1(db: &ModelV2ImperialDB, thread_type: &ThreadType) -> String {
        let class = match thread_type {
            ThreadType::Male => &db.class_m,
            ThreadType::Female => &db.class_f,
        };

        format!("{} - {} {} - {}", db.diameter, db.tpi, db.series_designation, class)
    }

    fn generate_designation2(db: &ModelV2ImperialDB, thread_type: &ThreadType) -> String {
        let class = match thread_type {
            ThreadType::Male => &db.class_m,
            ThreadType::Female => &db.class_f,
        };

        format!("({} - {} {} - {})", db.diameter_2, db.tpi, db.series_designation, class)
    }

    fn format_description(is_male: bool, language: &Language) -> String {
        let thread_type = match language {
            Language::Ru => {
                if is_male {
                    "наружная"
                } else {
                    "внутренняя"
                }
            }
            Language::En => {
                if is_male {
                    "external"
                } else {
                    "internal"
                }
            }
        };

        match language {
            Language::Ru => format!("Унифицированная цилиндрическая {} резьба", thread_type),
            Language::En => format!("Unified cylindrical {} thread", thread_type),
        }
    }

    fn format_unit(language: &Language, unit: &Unit) -> String {
        let unit_name = match unit {
            Unit::Mm => match language {
                Language::Ru => "миллиметр",
                Language::En => "millimeter",
            },
            Unit::Inch => match language {
                Language::Ru => "дюйм",
                Language::En => "inch",
            },
            Unit::Micron => match language {
                Language::Ru => "микрон",
                Language::En => "micron",
            },
        };
        match language {
            Language::Ru => format!("Единицы измерения: {}", unit_name),
            Language::En => format!("Units of measurement: {}", unit_name),
        }
    }

    fn map_main_info(
        is_male: bool,
        db: &ModelV2ImperialDB,
        request: &RequestV2ImperialInfo,
    ) -> Vec<ModelImperialAdditionalInfo> {
        let mut main_info = Vec::new();
        let formatter = |value: f64| {
            NumberFormatter::convert_and_round_to_string(value, &Unit::Inch, &request.units, request.precision, false)
        };
        let name = Self::get_localized_name(&request.language);

        // Thread Type
        main_info.push(ModelImperialAdditionalInfo {
            name: name("Thread Type", "Тип резьбы"),
            value: match (&request.language, is_male) {
                (Language::En, true) => "External Thread",
                (Language::En, false) => "Internal Thread",
                (Language::Ru, true) => "Наружная резьба",
                (Language::Ru, false) => "Внутренняя резьба",
            }
            .to_string(),
            description: None,
        });

        // Nominal diameter
        main_info.push(ModelImperialAdditionalInfo {
            name: name("Diameter (nominal)", "Диаметр (номинальный)"),
            // value: db.diameter.clone(),
            value: match request.units {
                Unit::Inch => format!("{} ({})", db.diameter.clone(), db.diameter_2),
                Unit::Mm => NumberFormatter::convert_and_round_to_string(
                    db.diameter_2,
                    &Unit::Inch,
                    &request.units,
                    request.precision,
                    false,
                )
                .to_string(),

                Unit::Micron => todo!(),
            }
            .to_string(),
            description: None,
        });

        // Threads per inch
        main_info.push(ModelImperialAdditionalInfo {
            name: name("Threads per inch (TPI)", "Число ниток на дюйм (TPI)"),
            value: db.tpi.to_string(),
            description: None,
        });

        // Series designation
        main_info.push(ModelImperialAdditionalInfo {
            name: name("Series designation", "Обозначение серии"),
            value: db.series_designation.clone(),
            description: None,
        });

        // Thread class
        main_info.push(ModelImperialAdditionalInfo {
            name: name("Thread class", "Класс точности резьбы"),
            value: if is_male {
                db.class_m.clone()
            } else {
                db.class_f.clone()
            },
            description: None,
        });

        // Thread pitch
        let pitch = 1.0 / db.tpi;
        main_info.push(ModelImperialAdditionalInfo {
            name: name("Thread pitch", "Шаг резьбы"),
            value: formatter(pitch),
            description: None,
        });

        // Thread depth
        let h = (3f64.sqrt() / 2.0) * pitch;
        let thread_depth = 0.625 * h; // 5H/8 for unified thread
        main_info.push(ModelImperialAdditionalInfo {
            name: name("Thread depth", "Глубина резьбы"),
            value: formatter(thread_depth),
            description: None,
        });
        if let Some(drill_info) = Self::calculate_hss_drill_size(db, request) {
            main_info.push(drill_info);
        }

        main_info
    }

    fn map_diameter_info(db: &ModelV2ImperialDB, request: &RequestV2ImperialInfo) -> Vec<ModelImperialDiameterInfo> {
        let mut result = Vec::new();
        let is_male = matches!(request.type_, ThreadType::Male);
        let name = Self::get_localized_name(&request.language);
        let formatter = |value: f64, diff: bool| {
            NumberFormatter::convert_and_round_to_string(value, &Unit::Inch, &request.units, request.precision, diff)
        };

        // Calculate basic thread dimensions
        let pitch = 1.0 / db.tpi;
        let h = (3f64.sqrt() / 2.0) * pitch;
        let d_basic = db.diameter_2;
        let major_diameter_basic = d_basic;
        let pitch_diameter_basic = d_basic - (2.0 * 0.375 * h); // d - 0.75H
        let minor_diameter_basic = d_basic - (2.0 * 0.625 * h); // d - 1.25H

        if is_male {
            // External (Male) Thread

            // Major diameter
            let max = db.major_diam_max_m;
            let min = db.major_diam_min_m;
            result.push(ModelImperialDiameterInfo {
                name: name("Major diameter (d)", "Наружный диаметр (d)"),
                max: formatter(max, false),
                min: formatter(min, false),
                avg: formatter((max + min) / 2.0, false),
                basic: formatter(major_diameter_basic, false),
                es: formatter(max - major_diameter_basic, true),
                ei: formatter(min - major_diameter_basic, true),
            });

            // Pitch diameter
            let max = db.pitch_diameter_max_m;
            let min = db.pitch_diameter_min_m;
            result.push(ModelImperialDiameterInfo {
                name: name("Pitch diameter (d2)", "Средний диаметр (d2)"),
                max: formatter(max, false),
                min: formatter(min, false),
                avg: formatter((max + min) / 2.0, false),
                basic: formatter(pitch_diameter_basic, false),
                es: formatter(max - pitch_diameter_basic, true),
                ei: formatter(min - pitch_diameter_basic, true),
            });

            // Minor diameter
            // For male threads, use calculated minor diameter
            let max = db.major_diam_max_m - 1.08253175 * pitch;

            result.push(ModelImperialDiameterInfo {
                name: name("Minor diameter (d1)", "Внутренний диаметр (d1)"),
                max: formatter(max, false),
                min: String::new(),
                avg: String::new(),
                basic: String::new(),
                es: String::new(),
                ei: String::new(), // No lower deviation for minor diameter
            });
            let max = db.unr_minor_diameter_max_m;
            result.push(ModelImperialDiameterInfo {
                name: name("Minor diameter UNR (d3)", "Внутренний диаметр UNR (d3)"),
                max: formatter(max, false),
                min: String::new(),
                avg: String::new(),
                basic: String::new(),
                es: String::new(),
                ei: String::new(), // No lower deviation for minor diameter
            });
        } else {
            // Internal (Female) Thread

            // Minor diameter
            let min = db.minor_diameter_min_f;
            let max = db.minor_diameter_max_f;
            result.push(ModelImperialDiameterInfo {
                name: name("Minor diameter (D1)", "Внутренний диаметр (D1)"),
                max: formatter(max, false),
                min: formatter(min, false),
                avg: formatter((max + min) / 2.0, false),
                basic: formatter(minor_diameter_basic, false),
                es: formatter(max - minor_diameter_basic, true),
                ei: formatter(min - minor_diameter_basic, true),
            });

            // Pitch diameter
            let min = db.pitch_diameter_min_f;
            let max = db.pitch_diameter_max_f;
            result.push(ModelImperialDiameterInfo {
                name: name("Pitch diameter (D2)", "Средний диаметр (D2)"),
                max: formatter(max, false),
                min: formatter(min, false),
                avg: formatter((max + min) / 2.0, false),
                basic: formatter(pitch_diameter_basic, false),
                es: formatter(max - pitch_diameter_basic, true),
                ei: formatter(min - pitch_diameter_basic, true),
            });

            // Major diameter
            // For female threads, there's only a minimum
            let min = db.major_diameter_min_f;
            result.push(ModelImperialDiameterInfo {
                name: name("Major diameter (D)", "Наружный диаметр (D)"),
                max: String::new(),
                min: formatter(min, false),
                avg: String::new(),
                basic: String::new(),
                es: String::new(),
                ei: String::new(),
            });
        }

        result
    }

    fn map_additional_info(
        db: &ModelV2ImperialDB,
        request: &RequestV2ImperialInfo,
    ) -> Vec<ModelImperialAdditionalInfo> {
        let mut result = Vec::new();
        let is_male = matches!(request.type_, ThreadType::Male);
        let name = Self::get_localized_name(&request.language);
        let formatter = |value: f64| {
            NumberFormatter::convert_and_round_to_string(value, &Unit::Inch, &request.units, request.precision, false)
        };

        // Thread pitch and basic calculations
        let tpi = db.tpi;
        let pitch = 1.0 / tpi;
        // Диаметр отверстия под резьбу

        // Thread profile angle
        result.push(ModelImperialAdditionalInfo {
            name: name("Thread profile angle", "Угол профиля резьбы"),
            value: "60°".to_string(),
            description: None,
        });
        // Allowance (only for external threads)
        if is_male && db.allowance != 0.0 {
            result.push(ModelImperialAdditionalInfo {
                name: name("Allowance", "Допуск"),
                value: formatter(db.allowance),
                description: None,
            });
        }

        let h = (3f64.sqrt() / 2.0) * pitch;
        result.push(ModelImperialAdditionalInfo {
            name: name("Basic height (H)", "Высота треугольника (H)"),
            value: formatter(h),
            description: None,
        });

        // Thread crest truncation
        let h_1_8 = h / 8.0;
        result.push(ModelImperialAdditionalInfo {
            name: name("Truncation at crest (H/8)", "Срез по вершине резьбы (H/8)"),
            value: formatter(h_1_8),
            description: None,
        });

        // Thread root truncation
        if !is_male {
            let h_5_16 = (5.0 / 16.0) * h;
            result.push(ModelImperialAdditionalInfo {
                name: name("Basic truncation at root (5H/16)", "Базовый срез у впадины (5H/16)"),
                value: formatter(h_5_16),
                description: None,
            });
        }
        // Basic thread depth
        let h_3_8 = (3.0 / 8.0) * h;
        result.push(ModelImperialAdditionalInfo {
            name: name("Basic thread depth (3H/8)", "Базовая глубина резьбы (3H/8)"),
            value: formatter(h_3_8),
            description: None,
        });

        // Max root material (external thread)
        if is_male {
            let h_9_16 = (9.0 / 16.0) * h;
            result.push(ModelImperialAdditionalInfo {
                name: name("Max material at root (9H/16)", "Макс. материал у впадины (9H/16)"),
                value: formatter(h_9_16),
                description: None,
            });
        }

        // Engagement lengths
        let short_length = 4.0 * pitch;
        let normal_length = 6.0 * pitch;
        let long_length = 10.0 * pitch;

        result.push(ModelImperialAdditionalInfo {
            name: name("Length of Engagement (short)", "Длина свинчивания (короткая)"),
            value: formatter(short_length),
            description: None,
        });

        result.push(ModelImperialAdditionalInfo {
            name: name("Length of Engagement (normal)", "Длина свинчивания (нормальная)"),
            value: formatter(normal_length),
            description: None,
        });

        result.push(ModelImperialAdditionalInfo {
            name: name("Length of Engagement (long)", "Длина свинчивания (длинная)"),
            value: formatter(long_length),
            description: None,
        });

        result
    }

    fn get_localized_name(language: &Language) -> impl Fn(&str, &str) -> String + '_ {
        move |en, ru| match language {
            Language::Ru => ru.to_string(),
            Language::En => en.to_string(),
        }
    }
    // Отдельный метод для расчета и форматирования размера сверла HSS
    fn calculate_hss_drill_size(
        db: &ModelV2ImperialDB,
        request: &RequestV2ImperialInfo,
    ) -> Option<ModelImperialAdditionalInfo> {
        // Проверяем, нужно ли вообще показывать информацию о сверле
        let is_male = matches!(request.type_, ThreadType::Male);
        if is_male {
            return None; // Для наружной резьбы не показываем
        }

        // Показываем информацию о сверле только если единицы измерения - миллиметры
        // Отключаем показ для дюймов и микрон
        match request.units {
            Unit::Mm => {}                            // Продолжаем выполнение для миллиметров
            Unit::Inch | Unit::Micron => return None, // Не показываем для дюймов и микрон
        }

        // Показываем информацию о сверле только если диаметр не превышает 30 мм
        if db.diameter_2 * 25.4 > 50.0 {
            return None;
        }

        // Базовый диаметр отверстия под нарезание резьбы (в дюймах)
        let min_diameter_mm = db.minor_diameter_min_f * 25.4;

        // Конвертируем в миллиметры для применения допуска h9
        let max_diameter_mm = db.minor_diameter_max_f * 25.4;

        // Вычисляем допуск h9 в зависимости от диаметра
        // Формулы для допуска h9 согласно ISO 286-2

        let h9_tolerance_mm = if min_diameter_mm <= 3.0 {
            0.025
        } else if min_diameter_mm <= 6.0 {
            0.030
        } else if min_diameter_mm <= 10.0 {
            0.036
        } else if min_diameter_mm <= 18.0 {
            0.043
        } else if min_diameter_mm <= 30.0 {
            0.052
        } else {
            0.062 // для диаметров от 30 до 50 мм
        };

        // 1. Берем минимальный диаметр и округляем до сверла (с шагом 0.1 мм)
        let min_possible_drill_size = (min_diameter_mm * 10.0).trunc() / 10.0;

        let max_possible_drill_size = (max_diameter_mm * 10.0).trunc() / 10.0;

        let mut suitable_drill_sizes = Vec::new();
        // величиваем допуск для безопасности
        let a = min_possible_drill_size + h9_tolerance_mm;

        if a > min_diameter_mm {
            suitable_drill_sizes.push(min_possible_drill_size);
        } else {
            suitable_drill_sizes.push(min_possible_drill_size + 0.1);
        }

        let b = max_possible_drill_size + h9_tolerance_mm * 2.0;
        if b < max_diameter_mm {
            suitable_drill_sizes.push(max_possible_drill_size);
        } else {
            suitable_drill_sizes.push(max_possible_drill_size - 0.1);
        }

        let first = suitable_drill_sizes[0];
        let last = *suitable_drill_sizes.last().unwrap_or(&first);

        // Получаем локализованное имя
        let name = Self::get_localized_name(&request.language);

        // Форматируем значения диапазона в зависимости от требуемых единиц
        // Для миллиметров форматируем с одним десятичным знаком
        let lower_formatted = format!("{:.1}", first);
        let upper_formatted = format!("{:.1}", last);

        // Формируем диапазон через тире
        let value_range = if lower_formatted == upper_formatted {
            format!("⌀{}", lower_formatted)
        } else {
            format!("⌀{} - ⌀{}", lower_formatted, upper_formatted)
        };

        // Возвращаем готовую структуру с информацией о сверле
        Some(ModelImperialAdditionalInfo {
            name: name("HSS Drill Size", "Размер сверла HSS"),
            value: value_range,
            description: None,
        })
    }
}
