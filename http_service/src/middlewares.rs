use axum::{body::Body, http::Request, middleware, response::IntoResponse};
use metrics::{counter, histogram};
use std::time::Instant;
use tracing::{info, info_span, Instrument};

pub async fn tracing_middleware(req: Request<Body>, next: middleware::Next) -> impl IntoResponse {
    let path = req.uri().path().to_string();
    let method = req.method().to_string();

    let request_id = uuid::Uuid::now_v7();

    let span = info_span!(
        "http_request",
        request_id = %request_id,
        method = %method,
        path = %path
    );

    let response = next.run(req).instrument(span).await;

    response
}

pub async fn logging_middleware(req: Request<Body>, next: middleware::Next) -> impl IntoResponse {
    info!(event = "http_request_start", "Request started");

    let start = Instant::now();
    let response = next.run(req).await;
    let duration = start.elapsed();
    let duration = duration.as_secs_f64();
    let status_code = response.status().as_u16();

    info!(
        event = "http_request_complete",
        duration = %duration,
        status = %status_code,
        "Request completed"
    );

    response
}

pub async fn metrics_middleware(req: Request<Body>, next: middleware::Next) -> impl IntoResponse {
    let path = req.uri().path().to_string();
    let method = req.method().to_string();

    let start = Instant::now();

    let response = next.run(req).await;

    let duration = start.elapsed();
    let duration = duration.as_secs_f64();
    let status_code = response.status().as_u16();

    counter!(
        "http_request_total",
        "path" => path.clone(),
        "method" => method.clone(),
        "status_code" => status_code.to_string()
    )
    .increment(1);
    histogram!(
        "http_request_duration_seconds",
        "path" => path,
        "method" => method,
        "status_code" => status_code.to_string()
    )
    .record(duration);

    response
}
