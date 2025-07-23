mod features;
mod shared;
use axum::{http::Request, routing::get, Router};
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use crate::shared::logging::{init::init_logging, structs::LogConfig, enums::LogLevel};

use crate::shared::{
    database::{migrations::run_migrations, service::PostgresService},
    logging, middleware,
    setting::models::{app_config::AppConfig, app_env::AppEnv, app_setting::AppSettings, app_state::AppState},
};

use tower_http::trace::TraceLayer;
mod analytics;

mod test;
use tower_http::cors::{Any, CorsLayer};

pub mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize application settings and logging
    let settings: Arc<AppSettings> = Arc::new(init_app().await);
    // Connect to databases
    let postgres_service = Arc::new(initialize_database(settings.clone()).await?);
    let server_address = format!("{}:{}", settings.env.server_address, settings.env.server_port)
        .parse()
        .expect("Invalid server address configuration");

    // Create application state with all services and dependencies
    let app_state = Arc::new(AppState::new(settings.clone(), postgres_service).await);

    // Create API router using app_state
    let app_router = create_application_router(app_state);

    // Start HTTP server
    start_http_server(app_router, server_address).await;

    Ok(())
}

async fn init_app() -> AppSettings {
    let environment = AppEnv::new();
    let config = AppConfig::new(&environment.env);
    let app_settings = AppSettings {
        config,
        env: environment,
    };

    // Setup logging with configured level
    let log_level = LogLevel::from(app_settings.config.logging.level.as_str());
    let log_config = LogConfig { level: log_level };
    init_logging(log_config)
        .expect("Failed to initialize logger");

    log_info!("Инициализация приложения с конфигурацией: {:?}", app_settings.config);

    // Log application startup information
    log_info!("Starting application...");
    log_info!("Current environment: {}", app_settings.env.env);

    if app_settings.env.is_development() {
        log_info!("Running in local or dev mode");
        log_debug!("Configuration details: {:#?}", app_settings);
    } else {
        log_info!("Running in production mode");
    }

    app_settings
}

fn create_application_router(app_state: Arc<AppState>) -> Router {
    use axum::routing::{get, post};
    let pool = app_state.postgres_service.connection.pool().clone();

    // Error reports router with state
    let error_reports_router = Router::new()
        .route(
            "/v1/error_reports/",
            post(crate::features::error_reports::handlers::create_error_report),
        )
        .with_state(pool.clone());

    // Main router with extension-based routes
    let main_router = Router::new()
        // === V1 METRIC ROUTES ===
        .route(
            "/v1/metric/diameters",
            get(crate::features::metric::v1::diameters::handler::diameters),
        )
        .route("/v1/metric/pitch", get(crate::features::metric::v1::pitch::pitch))
        .route(
            "/v1/metric/tolerance",
            get(crate::features::metric::v1::tolerance::tolerance),
        )
        .route(
            "/v1/metric/info",
            get(crate::features::metric::v1::info::handler::info),
        )
        .route("/v1/metric/svg", get(crate::features::metric::v1::svg::handler::svg))
        // === V1 IMPERIAL ROUTES ===
        .route(
            "/v1/imperial/diameters",
            get(crate::features::imperial::v1::diameters::handler::handle),
        )
        .route(
            "/v1/imperial/tolerance",
            get(crate::features::imperial::v1::tolerance::handler::handle),
        )
        .route(
            "/v1/imperial/info",
            get(crate::features::imperial::v1::info::handler::handle),
        )
        .route(
            "/v1/imperial/svg-annotations",
            get(crate::features::imperial::v1::svg_annotations::handlers::handler_get_svg_annotations::handle),
        )
        .route(
            "/v1/imperial/svg-dimensions",
            get(crate::features::imperial::v1::svg_dimensions::handlers::handler_get_svg_dimensions::handle),
        )
        // === V1 TRAPEZOIDAL ROUTES ===
        .route(
            "/v1/trapezoidal/diameters",
            get(crate::features::trapezoidal::v1::diameters::handler::handle),
        )
        .route(
            "/v1/trapezoidal/tolerance",
            get(crate::features::trapezoidal::v1::tolerance::handler::handle),
        )
        .route(
            "/v1/trapezoidal/info",
            get(crate::features::trapezoidal::v1::info::handler::handle),
        )
        .route(
            "/v1/trapezoidal/svg-dimensions",
            get(crate::features::trapezoidal::v1::svg_dimensions::handlers::handler_get_svg_dimensions::handle),
        )
        .route(
            "/v1/trapezoidal/svg-annotations",
            get(crate::features::trapezoidal::v1::svg_annotations::handlers::handler_get_svg_annotations::handle),
        )
        // === V1 PIPE ROUTES ===
        .route(
            "/v1/pipe/diameters",
            get(crate::features::pipe::v1::diameters::handler::handle),
        )
        .route(
            "/v1/pipe/info",
            get(crate::features::pipe::v1::info::handler::handle),
        )
        // === V2 IMPERIAL ROUTES ===
        .route(
            "/v2/imperial/info",
            get(crate::features::imperial::v2::info::handler::handle),
        )
        // === SYSTEM ROUTES ===
        .route("/test", get(crate::features::test::test))
        .route(
            "/health",
            get({
                let app_state = Arc::clone(&app_state);
                move || async move { app_state.health_handler.get_health().await }
            }),
        )
        .layer(axum::Extension(pool));

    // Combine routers
    Router::new()
        .merge(main_router)
        .merge(error_reports_router)
        // === MIDDLEWARE ===
        .layer(middleware::create_cors())
        .layer(middleware::create_trace())
        .layer(axum::Extension(app_state))
}

/// Starts the HTTP server on the specified address
async fn start_http_server(app: Router, addr: SocketAddr) {
    log_info!("Starting HTTP server on {}", addr);

    let listener = match TcpListener::bind(addr).await {
        Ok(listener) => listener,
        Err(err) => {
            log_error!("Failed to bind to address {}: {}", addr, err);
            panic!("Cannot start server: {}", err);
        }
    };

    log_info!("Server started successfully, now accepting connections");

    if let Err(err) = axum::serve(listener, app).await {
        log_error!("Server error: {}", err);
        panic!("Server failed: {}", err);
    }
}

async fn initialize_database(settings: Arc<AppSettings>) -> Result<PostgresService, Box<dyn std::error::Error>> {
    log_info!("Initializing database connections...");

    let postgres_service = PostgresService::new(&settings).await?;

    log_info!("Running database migrations...");
    run_migrations(postgres_service.connection.pool()).await?;

    Ok(postgres_service)
}
