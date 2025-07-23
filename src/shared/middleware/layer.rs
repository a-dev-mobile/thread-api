
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use tower_http::cors::{Any, CorsLayer};
use crate::shared::utils::http;
use crate::{log_info, log_debug};

/// Создаёт и настраивает middleware для логирования HTTP-запросов с использованием собственной системы логирования
pub fn create_trace<B>() -> tower::util::MapRequestLayer<impl Fn(Request<B>) -> Request<B> + Clone> {
    tower::util::MapRequestLayer::new(|request: Request<B>| {
        // Пропускаем логирование для запроса GET /test
        if request.uri().path() == "/test" {
            return request;
        }

        // Извлечение данных о запросе
        let method = request.method().clone();
        let uri = request.uri().clone();
        let version = request.version();
        let client_ip = http::get_client_ip(&request);
        
        // Извлечение заголовков
        let user_agent = request
            .headers()
            .get("user-agent")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("unknown");
        
        let referer = request
            .headers()
            .get("referer")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("unknown");
        
        let accept_language = request
            .headers()
            .get("accept-language")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("unknown");

        // Сокращенное логирование на уровне INFO
        log_info!("{} {} - {}", method, uri, client_ip);

        // Подробное логирование на уровне DEBUG
        log_debug!(
            "Request details: {} {} {:?} - IP: {}, User-Agent: {}, Referer: {}, Accept-Language: {}",
            method, uri, version, client_ip, user_agent, referer, accept_language
        );

        request
    })
}

pub fn create_cors() -> CorsLayer {
    // Настройка CORS
    CorsLayer::new()
        .allow_origin(Any) // Разрешить любые источники.
        .allow_methods(Any) // Разрешить любые HTTP-методы
        .allow_headers(Any) // Разрешить любые заголовки
}
