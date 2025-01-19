use anyhow::{Context, Result};
use http_service::{create_router, setup_metrics, Config};
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::from_env();

    config.setup_logging();

    let metrics_context = setup_metrics().context("Failed to setup metrics")?;

    let router = create_router(metrics_context);

    let listener = TcpListener::bind(format!("0.0.0.0:{}", config.port))
        .await
        .context("Failed to bind to port")?;

    info!("Listening on http://localhost:{}", config.port);

    axum::serve(listener, router)
        .await
        .context("Failed to serve")?;

    Ok(())
}
