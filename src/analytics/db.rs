use sqlx::PgPool;
use tracing::{error, info};

/// Добавляет новую запись в таблицу `popular_threads` или инкрементирует `usage_count`, если запись уже существует.
///
/// # Аргументы
///
/// * `pool` - Пул подключений к базе данных PostgreSQL.
/// * `full_thread_name` - Полное название резьбы.
///
/// # Возвращает
///
/// * `Result<(), sqlx::Error>` - Результат выполнения операции.
pub async fn add_or_increment_thread(
    pool: &PgPool,
    full_thread_name: String,
) -> Result<(), sqlx::Error> {
    // Вызов функции PostgreSQL
    sqlx::query!(
        "SELECT analytics.add_or_increment_thread($1)",
        full_thread_name
    )
    .execute(pool)
    .await?;
    Ok(())
}

/// Асинхронно вызывает функцию `add_or_increment_thread` и логирует результат.
///
/// # Аргументы
///
/// * `pool` - Пул подключений к базе данных PostgreSQL.
/// * `full_thread_name` - Полное название резьбы.
pub async fn handle_thread_analytics(pool: PgPool, full_thread_name: String) {
    match add_or_increment_thread(&pool, full_thread_name).await {
        Ok(_) => info!("Successfully updated usage_count for designation."),
        Err(e) => error!("Failed to update usage_count: {}", e),
    }
}
