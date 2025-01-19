use anyhow::Result;
use axum::{
    body::Body,
    extract::State,
    http::{header::CONTENT_TYPE, Response, StatusCode},
    response::IntoResponse,
};
use metrics::{describe_counter, describe_histogram};
use metrics_exporter_prometheus::PrometheusHandle;
use metrics_process::Collector;

pub const METRICS_CONTENT_TYPE: &str = "text/plain; version=0.0.4";

#[derive(Clone)]
pub struct MetricsContext {
    handle: PrometheusHandle,
    collector: Collector,
}

pub fn setup_metrics() -> Result<MetricsContext> {
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

    let collector = Collector::default();
    collector.describe();

    let handle = metrics_exporter_prometheus::PrometheusBuilder::new().install_recorder()?;

    Ok(MetricsContext { handle, collector })
}

pub async fn serve_metrics(State(context): State<MetricsContext>) -> impl IntoResponse {
    context.collector.collect();

    Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, METRICS_CONTENT_TYPE)
        .body(Body::from(context.handle.render()))
        .unwrap()
}
