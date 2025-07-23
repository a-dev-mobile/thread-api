// src/error/types.rs - Определение типов ошибок
use axum::http::StatusCode;
use serde_json::json;
use std::fmt;

#[derive(Debug, Clone)]
pub enum AppError {
    // Критичные системные ошибки
    ConfigurationError { message: String },
    EnvironmentError { variable: String },
    InternalError { message: String },
    DatabaseConnectionError { message: String },
    MigrationError { message: String },
    
    // Прикладные ошибки
    BadRequest(String),
    InvalidThreadParams(String),
    ThreadDataNotFound { diameter: f64, pitch: f64 },
    SvgTemplateNotFound(String),
    InvalidSvgTemplate(String),
    DatabaseError(String),
    FileSystemError(String),
    ValidationError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::ConfigurationError { message } => {
                write!(f, "Ошибка конфигурации: {}", message)
            }
            AppError::EnvironmentError { variable } => {
                write!(f, "Переменная окружения '{}' не установлена", variable)
            }
            AppError::InternalError { message } => {
                write!(f, "Внутренняя ошибка: {}", message)
            }
            AppError::DatabaseConnectionError { message } => {
                write!(f, "Ошибка подключения к базе данных: {}", message)
            }
            AppError::MigrationError { message } => {
                write!(f, "Ошибка миграции базы данных: {}", message)
            }
            AppError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            AppError::InvalidThreadParams(msg) => write!(f, "Invalid thread parameters: {}", msg),
            AppError::ThreadDataNotFound { diameter, pitch } => {
                write!(f, "Thread data not found for diameter {} and pitch {}", diameter, pitch)
            }
            AppError::SvgTemplateNotFound(path) => write!(f, "SVG template not found: {}", path),
            AppError::InvalidSvgTemplate(msg) => write!(f, "Invalid SVG template format: {}", msg),
            AppError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            AppError::FileSystemError(msg) => write!(f, "File system error: {}", msg),
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

impl AppError {
    pub fn error_code(&self) -> &'static str {
        match self {
            AppError::ConfigurationError { .. } | 
            AppError::EnvironmentError { .. } | 
            AppError::InternalError { .. } |
            AppError::DatabaseConnectionError { .. } |
            AppError::MigrationError { .. } => "SYSTEM_ERROR",
            AppError::BadRequest(_) => "BAD_REQUEST",
            AppError::InvalidThreadParams(_) => "INVALID_THREAD_PARAMS",
            AppError::ThreadDataNotFound { .. } => "THREAD_NOT_FOUND",
            AppError::SvgTemplateNotFound(_) => "SVG_TEMPLATE_NOT_FOUND",
            AppError::InvalidSvgTemplate(_) => "INVALID_SVG_TEMPLATE",
            AppError::DatabaseError(_) => "DATABASE_ERROR",
            AppError::FileSystemError(_) => "FILE_SYSTEM_ERROR",
            AppError::ValidationError(_) => "VALIDATION_ERROR",
        }
    }

    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::ConfigurationError { .. } | 
            AppError::EnvironmentError { .. } | 
            AppError::InternalError { .. } |
            AppError::DatabaseConnectionError { .. } |
            AppError::MigrationError { .. } |
            AppError::DatabaseError(_) |
            AppError::FileSystemError(_) |
            AppError::InvalidSvgTemplate(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::BadRequest(_) |
            AppError::InvalidThreadParams(_) |
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
            AppError::ThreadDataNotFound { .. } |
            AppError::SvgTemplateNotFound(_) => StatusCode::NOT_FOUND,
        }
    }

    pub fn details(&self) -> serde_json::Value {
        match self {
            AppError::EnvironmentError { variable } => json!({
                "variable": variable
            }),
            AppError::DatabaseConnectionError { message } => json!({
                "message": message
            }),
            AppError::ThreadDataNotFound { diameter, pitch } => json!({
                "diameter": diameter,
                "pitch": pitch
            }),
            AppError::BadRequest(msg) |
            AppError::InvalidThreadParams(msg) |
            AppError::SvgTemplateNotFound(msg) |
            AppError::InvalidSvgTemplate(msg) |
            AppError::DatabaseError(msg) |
            AppError::FileSystemError(msg) |
            AppError::ValidationError(msg) => json!({
                "message": msg
            }),
            _ => json!({}),
        }
    }
}
