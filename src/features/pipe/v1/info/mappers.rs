use super::models::{request_pipe_info::RequestPipeInfo, response_pipe_info::ResponsePipeInfo};
use crate::features::pipe::v1::common::models::model_pipe_db::ModelPipeDB;
use crate::features::pipe::v1::common::models::{ModelPipeAdditionalInfo, ModelPipeDiameterInfo};
use crate::shared::enums::{Language, Unit};
use crate::shared::utils::number::NumberFormatter;

impl ResponsePipeInfo {
    pub fn from_data(db: ModelPipeDB, request: &RequestPipeInfo) -> Self {
        let is_male = db.class_name.is_some();

        Self {
            designation1: Self::format_designation1(&db),
            designation2: Self::format_designation2(&db, is_male),
            description: Self::format_description(is_male, &request.language),
            unit: Self::format_unit(&request.language, &request.units),
            main_info: Self::map_main_info(is_male, &db, request),
            diameter_info: Self::map_diameter_info(&db, request),
            additional_info: Self::map_additional_info(&db, request),
        }
    }

    /// Форматирует первую строку обозначения (Designation1).
    fn format_designation1(db: &ModelPipeDB) -> String {
        format!("G {} - {}", db.designation_2, db.thread_per)
    }

    /// Форматирует вторую строку обозначения (Designation2) на основе типа резьбы.
    fn format_designation2(db: &ModelPipeDB, is_male: bool) -> String {
        let decimal_diam = if is_male {
            db.ex_major_dia_max.unwrap_or_default()
        } else {
            db.in_major_dia_min.unwrap_or_default()
        };
        format!("G {} x {}", decimal_diam, db.thread_pitch)
    }

    /// Возвращает описание резьбы на основе языка и типа (внешняя/внутренняя).
    fn format_description(is_male: bool, language: &Language) -> String {
        let thread_type = Self::get_thread_type(language, is_male);
        match language {
            Language::Ru => format!("G - Трубная цилиндрическая {}", thread_type),
            Language::En => format!("G - Cylindrical pipe {}", thread_type),
        }
    }

    /// Форматирует единицы измерения в зависимости от языка и выбранной единицы.
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

    /// Преобразует основные параметры резьбы в список дополнительной информации.
    fn map_main_info(is_male: bool, db: &ModelPipeDB, request: &RequestPipeInfo) -> Vec<ModelPipeAdditionalInfo> {
        let mut main_info = Vec::new();
        let formatter = |value: f64| {
            NumberFormatter::convert_and_round_to_string(
                value,
                &Unit::Mm,
                &request.units,
                Some(request.precision),
                false,
            )
        };
        let name = Self::get_localized_name(&request.language);

        // Тип резьбы
        main_info.push(ModelPipeAdditionalInfo {
            name: name("Thread Type", "Тип резьбы"),
            value: Self::get_thread_type(&request.language, is_male),
            description: None,
        });

        // Номинальный диаметр
        main_info.push(ModelPipeAdditionalInfo {
            name: name("Diameter (nominal)", "Диаметр (номинальный)"),
            value: db.designation_2.clone(),
            description: None,
        });

        // Число ниток на дюйм
        main_info.push(ModelPipeAdditionalInfo {
            name: name("Threads per inch (TPI)", "Число ниток на дюйм (TPI)"),

            value: db.thread_per.to_string(),
            description: None,
        });

        // Класс резьбы (если есть)
        if let Some(class) = &db.class_name {
            main_info.push(ModelPipeAdditionalInfo {
                name: name("Thread class", "Класс точности резьбы"),
                value: class.clone(),
                description: None,
            });
        }

        // Шаг резьбы
        main_info.push(ModelPipeAdditionalInfo {
            name: name("Thread pitch", "Шаг резьбы"),
            value: formatter(db.thread_pitch),
            description: None,
        });

        // Глубина резьбы
        let thread_depth = if is_male {
            (db.ex_major_dia_max.unwrap_or_default() - db.ex_minor_dia_max.unwrap_or_default()) / 2.0
        } else {
            (db.in_major_dia_min.unwrap_or_default() - db.in_minor_dia_max.unwrap_or_default()) / 2.0
        };
        main_info.push(ModelPipeAdditionalInfo {
            name: name("Thread depth", "Глубина резьбы"),
            value: formatter(thread_depth),
            description: None,
        });

        main_info
    }

    /// Преобразует данные о диаметрах резьбы.
    fn map_diameter_info(db: &ModelPipeDB, request: &RequestPipeInfo) -> Vec<ModelPipeDiameterInfo> {
        let mut result = Vec::new();
        let is_male = db.class_name.is_some();
        let name = Self::get_localized_name(&request.language);
        let formatter = |value: f64, diff: bool| {
            NumberFormatter::convert_and_round_to_string(
                value,
                &Unit::Mm,
                &request.units,
                Some(request.precision),
                diff,
            )
        };

        if is_male {
            // Внешний большой диаметр
            let max = db.ex_major_dia_max.unwrap_or_default();
            let min = db.ex_major_dia_min.unwrap_or_default();
            result.push(ModelPipeDiameterInfo {
                max: formatter(max, false),
                min: formatter(min, false),
                avg: formatter((max + min) / 2.0, false),
                name: name("External diameter (d)", "Наружный диаметр (d)"),
                basic: formatter(max, false),
                es: String::new(),
                ei: formatter(min - max, true),
            });

            // Внешний средний диаметр
            let max = db.ex_pitch_diam_max.unwrap_or_default();
            let min = db.ex_pitch_diam_min.unwrap_or_default();
            result.push(ModelPipeDiameterInfo {
                max: formatter(max, false),
                min: formatter(min, false),
                avg: formatter((max + min) / 2.0, false),
                name: name("Pitch diameter (d2)", "Средний диаметр (d2)"),
                basic: formatter(max, false),
                es: String::new(),
                ei: formatter(min - max, true),
            });

            // Внешний малый диаметр
            result.push(ModelPipeDiameterInfo {
                max: formatter(db.ex_minor_dia_max.unwrap_or_default(), false),
                min: String::new(),
                avg: String::new(),
                name: name("Minor diameter (d1)", "Внутренний диаметр (d1)"),
                basic: String::new(),
                es: String::new(),
                ei: String::new(),
            });
        } else {
            // Внутренний малый диаметр
            let max = db.in_minor_dia_max.unwrap_or_default();
            let min = db.in_minor_dia_min.unwrap_or_default();
            result.push(ModelPipeDiameterInfo {
                max: formatter(max, false),
                min: formatter(min, false),
                avg: formatter((max + min) / 2.0, false),
                name: name("Minor diameter (D1)", "Внутренний диаметр (D1)"),
                basic: formatter(min, false),
                es: formatter(max - min, true),
                ei: String::new(),
            });

            // Диаметр отверстия под резьбу
            result.push(ModelPipeDiameterInfo {
                max: String::new(),
                min: String::new(),
                avg: String::new(),
                name: name("Thread Hole Diameter", "Диаметр отверстия под резьбу"),
                basic: formatter(db.in_tap_drill.unwrap_or_default(), false),
                es: String::new(),
                ei: String::new(),
            });

            // Внутренний средний диаметр
            let max = db.in_pitch_diam_max.unwrap_or_default();
            let min = db.in_pitch_diam_min.unwrap_or_default();
            result.push(ModelPipeDiameterInfo {
                max: formatter(max, false),
                min: formatter(min, false),
                avg: formatter((max + min) / 2.0, false),
                name: name("Pitch diameter (D2)", "Средний диаметр (D2)"),
                basic: formatter(min, false),
                es: formatter(max - min, true),
                ei: String::new(),
            });

            // Внутренний большой диаметр
            result.push(ModelPipeDiameterInfo {
                max: String::new(),
                min: formatter(db.in_major_dia_min.unwrap_or_default(), false),
                avg: String::new(),
                name: name("Major diameter (D)", "Наружный диаметр (D)"),
                basic: String::new(),
                es: String::new(),
                ei: String::new(),
            });
        }

        result
    }

    /// Преобразует дополнительные параметры резьбы.
    fn map_additional_info(db: &ModelPipeDB, request: &RequestPipeInfo) -> Vec<ModelPipeAdditionalInfo> {
        let mut result = Vec::new();
        let is_male = db.class_name.is_some();
        let name = Self::get_localized_name(&request.language);
        let formatter = |value: f64| {
            NumberFormatter::convert_and_round_to_string(
                value,
                &Unit::Mm,
                &request.units,
                Some(request.precision),
                false,
            )
        };
        // Описание класса точности (только для внешних резьб)
        if is_male {
            if let Some(class) = &db.class_name {
                result.push(ModelPipeAdditionalInfo {
                    name: name("Tolerance class", "Класс точности"),
                    value: match class.as_str() {
                        "A" => match request.language {
                            Language::En => "High (A)".to_string(),
                            Language::Ru => "Высокий (А)".to_string(),
                        },
                        "B" => match request.language {
                            Language::En => "Regular (B)".to_string(),
                            Language::Ru => "Cтандартный (B)".to_string(),
                        },
                        _ => "Unknown class".to_string(),
                    },
                    description: None,
                });
            }
        }

        // Угол профиля резьбы (фиксированный для G-резьбы)
        result.push(ModelPipeAdditionalInfo {
            name: name("Thread profile angle", "Угол профиля резьбы"),
            value: "55°".to_string(),
            description: None,
        });

        // Шаг резьбы (P) для расчётов
        let p = db.thread_pitch;

        // H - Высота основного треугольника
        let h = 0.960_491 * p;
        result.push(ModelPipeAdditionalInfo {
            name: name("Triangle height (H)", "Высота треугольника (H)"),
            value: formatter(h),
            description: None,
        });

        // h - Высота профиля резьбы
        let h_profile = 0.640_327 * p;
        result.push(ModelPipeAdditionalInfo {
            name: name("Profile height (h)", "Высота профиля (h)"),
            value: formatter(h_profile),
            description: None,
        });

        // r - Радиус округления
        let r = 0.137_329 * p;
        result.push(ModelPipeAdditionalInfo {
            name: name("Crest radius", "Радиус вершины"),
            value: formatter(r),
            description: None,
        });

        result.push(ModelPipeAdditionalInfo {
            name: name("Root radius", "Радиус впадины"),
            value: formatter(r),
            description: None,
        });

        // Полезная длина резьбы (оценка: 6 витков)
        let short_length = 4.0 * p;
        result.push(ModelPipeAdditionalInfo {
            name: name("Length of Engagement (short)", "Длина свинчивания (короткая)"),
            value: formatter(short_length),
            description: None,
        });
        // Нормальная длина (N) - 6 витков
        let normal_length = 6.0 * p;
        result.push(ModelPipeAdditionalInfo {
            name: name("Length of Engagement (normal)", "Длина свинчивания (нормальная)"),
            value: formatter(normal_length),
            description: None,
        });
        let long_length = 10.0 * p;
        result.push(ModelPipeAdditionalInfo {
            name: name("Length of Engagement (long)", "Длина свинчивания (длинная)"),
            value: formatter(long_length),
            description: None,
        });
        result
    }

    /// Возвращает тип резьбы (внешняя/внутренняя) на основе языка.
    fn get_thread_type(language: &Language, is_male: bool) -> String {
        match language {
            Language::Ru => if is_male {
                "Наружняя резьба"
            } else {
                "Внутренняя резьба"
            }
            .to_string(),
            Language::En => if is_male { "External Thread" } else { "Internal Thread" }.to_string(),
        }
    }

    /// Возвращает функцию для получения локализованных имен (En/Ru).
    fn get_localized_name(language: &Language) -> impl Fn(&str, &str) -> String + '_ {
        move |en, ru| match language {
            Language::Ru => ru.to_string(),
            Language::En => en.to_string(),
        }
    }
}
