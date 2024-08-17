use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use tokio::{self, signal};

use anyhow;
use tracing::{event, Level};
use tracing_subscriber::{fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

use serde::{self, Deserialize};
use toml;

#[derive(Deserialize)]
struct Config {
    server: ServerConfig,
    trace: TraceConfig,
}
#[derive(Deserialize)]
struct ServerConfig {
    host: String,
    port: String,
}
#[derive(Deserialize)]
struct TraceConfig {
    logging_level: String,
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    let config_toml_path = "config.toml";
    let config = std::fs::read_to_string(config_toml_path)?;

    let Config { server, trace } = toml::from_str(&config)?;

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::new(trace.logging_level))
        .init();

    let (host, port) = (server.host, server.port);

    event!(Level::DEBUG, "PORT: {port}, HOST: {host}");

    let app = Router::new()
        .route("/", get(home_handler))
        .route("/random_gif", get(random_gif));
    let listener = tokio::net::TcpListener::bind(format!("{host}:{port}")).await?;

    event!(Level::DEBUG, "Server start listening");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn home_handler() -> Result<Html<String>, impl IntoResponse> {
    event!(Level::DEBUG, "Client connected to home");

    let home_html_path = "src/public/home.html";

    let home_html_str = match tokio::fs::read_to_string(home_html_path).await {
        Ok(home_html_str) => home_html_str,
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Server error: {err}"),
            ))
        }
    };

    Ok(Html::from(home_html_str))
}

async fn random_gif() -> impl IntoResponse {
    Html("should get gif")
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
