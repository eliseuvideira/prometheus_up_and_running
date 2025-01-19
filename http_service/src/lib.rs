use axum::{middleware::from_fn, routing::get, Router};

mod config;
mod errors;
mod metrics;
mod middlewares;
mod routes;

pub use config::*;
pub use errors::*;
pub use metrics::*;
pub use middlewares::*;
pub use routes::*;

pub fn create_router(metrics_context: MetricsContext) -> Router {
    let application_routes = Router::new()
        .route("/", get(routes::hello_world))
        .route("/health", get(routes::health_check))
        .route("/error", get(routes::error_endpoint))
        .layer(from_fn(middlewares::metrics_middleware))
        .fallback(routes::not_found)
        .layer(from_fn(middlewares::logging_middleware))
        .layer(from_fn(middlewares::tracing_middleware));

    let metrics_routes = Router::new()
        .route("/metrics", get(metrics::serve_metrics))
        .with_state(metrics_context);

    Router::new()
        .merge(application_routes)
        .merge(metrics_routes)
}
