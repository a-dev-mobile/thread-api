use super::types::AppError;

// Только системные конверсии
impl From<sqlx::migrate::MigrateError> for AppError {
    fn from(err: sqlx::migrate::MigrateError) -> Self {
        AppError::MigrationError {
            message: err.to_string(),
        }
    }
}

// Конверсии для sqlx::Error
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match &err {
            // Критичные ошибки подключения к БД
            sqlx::Error::Configuration(_) | sqlx::Error::Io(_) | sqlx::Error::Tls(_) => {
                AppError::DatabaseConnectionError {
                    message: err.to_string(),
                }
            }
            // Остальные ошибки БД как прикладные
            _ => AppError::DatabaseError(err.to_string()),
        }
    }
}

// Конверсия для std::io::Error
impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::FileSystemError(err.to_string())
    }
}
