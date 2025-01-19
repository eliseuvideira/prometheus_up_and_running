use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use metrics::counter;
use tracing::error;

use crate::{Config, Environment};

pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response<Body> {
        counter!("app_error_total").increment(1);
        error!(
            event = "app_error",
            error = %self.0,
            cause = ?self.0.root_cause(),
            chain = ?self.0.chain().collect::<Vec<_>>(),
            "Error occurred"
        );

        let body = match Config::env() {
            Environment::Development | Environment::Test => Body::from(self.0.to_string()),
            _ => Body::empty(),
        };

        let response = Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(body)
            .unwrap();

        response
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(error: E) -> Self {
        Self(error.into())
    }
}
