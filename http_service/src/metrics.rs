use anyhow::Result;
use axum::{
    body::Body,
    extract::State,
    http::{header::CONTENT_TYPE, Response, StatusCode},
    response::IntoResponse,
};
use metrics::{describe_counter, describe_histogram};
use metrics_exporter_prometheus::PrometheusHandle;

pub const METRICS_CONTENT_TYPE: &str = "text/plain; version=0.0.4";

pub fn setup_metrics() -> Result<PrometheusHandle> {
    describe_counter!(
        "http_request_total",
        metrics::Unit::Count,
        "Total number of requests"
    );
    describe_histogram!(
        "http_request_duration_seconds",
        metrics::Unit::Seconds,
        "Time taken to process requests"
    );
    describe_counter!(
        "http_request_not_found",
        metrics::Unit::Count,
        "Total number of requests that returned 404"
    );
    describe_counter!(
        "app_error_total",
        metrics::Unit::Count,
        "Total number of errors"
    );

    metrics_exporter_prometheus::PrometheusBuilder::new()
        .install_recorder()
        .map_err(Into::into)
}

pub async fn serve_metrics(State(handle): State<PrometheusHandle>) -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, METRICS_CONTENT_TYPE)
        .body(Body::from(handle.render()))
        .unwrap()
}
