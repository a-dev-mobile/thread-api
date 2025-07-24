// src/utils.rs

use crate::shared::enums::Unit;

/// Утилитный модуль для работы с числами
pub struct NumberFormatter;

impl NumberFormatter {
    /// Возвращает число, округленное до заданной точности
    ///
    /// # Аргументы
    ///
    /// * `number` - число с плавающей точкой
    /// * `precision` - количество знаков после запятой
    ///
    /// # Примеры
    ///
    /// ```
    /// let rounded = NumberFormatter::round(3.14159, 2);
    /// assert_eq!(rounded, 3.14);
    /// ```
    pub fn round(number: f64, precision: usize) -> f64 {
        let factor = 10_f64.powi(precision as i32);
        (number * factor).round() / factor
    }

    /// Конвертирует число между микронами, миллиметрами и дюймами и округляет его до заданной точности
    ///
    /// # Аргументы
    ///
    /// * `value` - исходное значение
    /// * `from_units` - исходные единицы измерения
    /// * `to_units` - целевые единицы измерения
    /// * `precision` - опциональная точность после запятой
    ///
    /// # Примеры
    ///
    /// ```
    /// // Конвертация из микронов в миллиметры
    /// let mm = NumberFormatter::convert_and_round(1000.0, &Units::Micron, &Units::Mm, Some(3));
    /// assert_eq!(mm, 1.000);
    ///
    /// // Конвертация из микронов в дюймы
    /// let inches = NumberFormatter::convert_and_round(25400.0, &Units::Micron, &Units::Inch, Some(3));
    /// assert_eq!(inches, 1.000);
    ///
    /// // Конвертация из дюймов в микроны
    /// let microns = NumberFormatter::convert_and_round(1.0, &Units::Inch, &Units::Micron, Some(1));
    /// assert_eq!(microns, 25400.0);
    /// ```
    pub fn convert_and_round(value: f64, from_units: &Unit, to_units: &Unit, precision: Option<usize>) -> f64 {
        if from_units == to_units {
            return match precision {
                Some(p) => Self::round(value, p),
                None => value,
            };
        }

        // Константы для конвертации
        const MICRONS_PER_MM: f64 = 1000.0;
        const MM_PER_INCH: f64 = 25.4;
        const MICRONS_PER_INCH: f64 = MICRONS_PER_MM * MM_PER_INCH;

        let converted = match (from_units, to_units) {
            // Конвертации с микронами
            (Unit::Micron, Unit::Mm) => value / MICRONS_PER_MM,
            (Unit::Micron, Unit::Inch) => value / MICRONS_PER_INCH,
            (Unit::Mm, Unit::Micron) => value * MICRONS_PER_MM,
            (Unit::Inch, Unit::Micron) => value * MICRONS_PER_INCH,

            // Конвертации между мм и дюймами
            (Unit::Inch, Unit::Mm) => value * MM_PER_INCH,
            (Unit::Mm, Unit::Inch) => value / MM_PER_INCH,

            _ => value, // Этот случай уже обработан в начале функции
        };

        match precision {
            Some(p) => Self::round(converted, p),
            None => converted,
        }
    }
    // Обновленная версия convert_and_round_to_string
    pub fn convert_and_round_to_string(
        value: f64,
        from_units: &Unit,
        to_units: &Unit,
        precision: Option<usize>,
        show_plus: bool, // Параметр для отображения плюса
    ) -> String {
        let number = Self::convert_and_round(value, from_units, to_units, precision);

        // Если число равно 0 и show_plus = true, возвращаем пустую строку
        if show_plus && number == 0.0 {
            return String::new();
        }

        let mut result = Self::format_number_trim_zeros(number, precision);

        // Добавляем плюс к положительным числам, если show_plus = true
        if show_plus && number > 0.0 {
            result.insert(0, '+');
        }

        result
    }

    /// Удаляет лишние нули после запятой и форматирует число в строку
    pub fn format_number_trim_zeros(value: f64, precision: Option<usize>) -> String {
        let formatted = match precision {
            Some(p) => format!("{:.*}", p, value),
            None => value.to_string(),
        };

        // Если число целое - убираем десятичную часть
        if formatted.contains('.') {
            formatted.trim_end_matches('0').trim_end_matches('.').to_string()
        } else {
            formatted
        }
    }
}
