use axum::http::HeaderMap;

/// Поддерживаемые языки приложения
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Language {
    Russian,
    English,
}

impl Language {
    /// Возвращает код языка
    pub fn code(&self) -> &'static str {
        match self {
            Language::Russian => "ru",
            Language::English => "en",
        }
    }

    /// Проверяет, является ли язык русским
    pub fn is_russian(&self) -> bool {
        matches!(self, Language::Russian)
    }

    /// Проверяет, является ли язык английским
    pub fn is_english(&self) -> bool {
        matches!(self, Language::English)
    }
}

impl From<&str> for Language {
    fn from(lang_code: &str) -> Self {
        let normalized = lang_code.to_lowercase();
        if normalized.starts_with("ru") {
            Language::Russian
        } else {
            Language::English
        }
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code())
    }
}

/// Утилиты для работы с языками
pub struct LanguageUtils;

impl LanguageUtils {
    /// Извлекает язык из заголовка Accept-Language
    ///
    /// # Примеры
    /// - "ru-RU,ru;q=0.9,en;q=0.8" -> Language::Russian
    /// - "en-US,en;q=0.9" -> Language::English
    /// - None или некорректное значение -> Language::English (по умолчанию)
    pub fn extract_from_headers(headers: &HeaderMap) -> Language {
        headers
            .get("accept-language")
            .and_then(|value| value.to_str().ok())
            .map(Self::parse_accept_language)
            .unwrap_or(Language::English)
    }

    /// Извлекает язык из строки Accept-Language
    pub fn parse_accept_language(accept_language: &str) -> Language {
        // Берем первый язык из списка (например, "ru-RU,ru;q=0.9,en;q=0.8" -> "ru")
        let primary_lang = accept_language
            .split(',')
            .next()
            .unwrap_or("en")
            .split('-')
            .next()
            .unwrap_or("en")
            .split(';')
            .next()
            .unwrap_or("en")
            .trim()
            .to_lowercase();

        Language::from(primary_lang.as_str())
    }

    /// Выбирает локализованный текст на основе языка
    ///
    /// # Аргументы
    /// - `language` - язык для выбора
    /// - `ru_text` - текст на русском языке
    /// - `en_text` - текст на английском языке
    ///
    /// # Возвращает
    /// Соответствующий текст для указанного языка
    pub fn localize_text(language: &Language, ru_text: &str, en_text: &str) -> String {
        match language {
            Language::Russian => ru_text.to_string(),
            Language::English => en_text.to_string(),
        }
    }

    /// Выбирает локализованный текст из пары Option<String>
    pub fn localize_optional_text(language: &Language, ru_text: Option<&str>, en_text: Option<&str>) -> Option<String> {
        match language {
            Language::Russian => ru_text.map(|s| s.to_string()),
            Language::English => en_text.map(|s| s.to_string()),
        }
    }

    /// Возвращает язык по умолчанию
    pub fn default() -> Language {
        Language::English
    }

    /// Проверяет, поддерживается ли указанный код языка
    pub fn is_supported(lang_code: &str) -> bool {
        matches!(lang_code.to_lowercase().as_str(), "ru" | "en")
    }

    /// Возвращает список всех поддерживаемых языков
    pub fn supported_languages() -> Vec<Language> {
        vec![Language::Russian, Language::English]
    }
}
