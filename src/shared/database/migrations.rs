use sqlx::{
    migrate::{MigrateError, Migrator},
    Executor, PgPool,
};
use std::path::Path;
use crate::{log_error, log_info};

pub async fn run_migrations(pool: &PgPool) -> Result<(), MigrateError> {
    log_info!("Running database migrations...");

    // Читаем миграции во время выполнения
    let migrations_path = Path::new("./migrations");

    match Migrator::new(migrations_path).await {
        Ok(migrator) => {
            // Показываем какие миграции найдены
            let migrations = migrator.iter().collect::<Vec<_>>();
            log_info!("Found {} migration(s):", migrations.len());
            for migration in &migrations {
                log_info!("  - {} : {}", migration.version, migration.description);
            }

            match migrator.run(pool).await {
                Ok(_) => {
                    log_info!("✅ Database migrations completed successfully");

                    // Проверяем результат
                    let applied_migrations: Vec<(i64, String)> = sqlx::query_as(
                        "SELECT version, description FROM _sqlx_migrations ORDER BY version",
                    )
                    .fetch_all(pool)
                    .await
                    .map_err(MigrateError::Execute)?;

                    log_info!("Applied migrations:");
                    for (version, description) in applied_migrations {
                        log_info!("  ✓ {} : {}", version, description);
                    }

                    Ok(())
                }
                Err(e) => {
                    log_error!("Failed to run migrations: {}", e);
                    Err(e)
                }
            }
        }
        Err(e) => {
            log_error!(
                "Failed to create migrator from path {:?}: {}",
                migrations_path, e
            );
            Err(e)
        }
    }
}

// Функция для быстрой проверки новых миграций без перезапуска
pub async fn check_pending_migrations(pool: &PgPool) -> Result<(), MigrateError> {
    log_info!("Checking for pending migrations...");

    let migrations_path = Path::new("./migrations");
    let migrator = Migrator::new(migrations_path).await?;

    // Получаем список всех миграций
    let all_migrations = migrator.iter().collect::<Vec<_>>();

    // Получаем примененные миграции
    let applied: Vec<(i64,)> =
        sqlx::query_as("SELECT version FROM _sqlx_migrations ORDER BY version")
            .fetch_all(pool)
            .await
            .map_err(MigrateError::Execute)?;

    let applied_versions: std::collections::HashSet<i64> =
        applied.into_iter().map(|(v,)| v).collect();

    let pending: Vec<_> = all_migrations
        .iter()
        .filter(|m| !applied_versions.contains(&m.version))
        .collect();

    if pending.is_empty() {
        log_info!("✅ No pending migrations");
    } else {
        log_info!("📋 Found {} pending migration(s):", pending.len());
        for migration in pending {
            log_info!("  - {} : {}", migration.version, migration.description);
        }
    }

    Ok(())
}
