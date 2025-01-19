use anyhow::{Context, Result};
use axum::{
    body::Body,
    http::{Response, StatusCode},
};
use metrics::counter;

use crate::AppError;

pub async fn hello_world() -> Result<String, AppError> {
    Ok("Hello, world!".to_string())
}

pub async fn health_check() -> Result<Response<Body>, AppError> {
    let response = Response::builder()
        .status(StatusCode::OK)
        .body(Body::empty())?;

    Ok(response)
}

pub async fn error_endpoint() -> Result<String, AppError> {
    let x: u16 = "120a".parse().context("Failed to parse u16")?;

    Ok(x.to_string())
}

pub async fn not_found() -> Result<Response<Body>, AppError> {
    counter!("http_request_not_found").increment(1);

    let response = Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::empty())?;

    Ok(response)
}
