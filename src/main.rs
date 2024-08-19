use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use tenor::queries::types::random_cat_gif_query;
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

pub mod representation;
pub mod tenor;

use representation::html::{get_home_html, get_random_cat_gif_html};

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
        .route("/random_cat_gif", get(random_gif));
    let listener = tokio::net::TcpListener::bind(format!("{host}:{port}")).await?;

    event!(Level::DEBUG, "Server start listening");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn home_handler() -> Result<Html<String>, impl IntoResponse> {
    event!(Level::DEBUG, "Client connected to `/`");

    let home_html = match get_home_html().await {
        Ok(home_html) => home_html,
        Err(err) => return Err(err),
    };

    Ok(home_html)
}

async fn random_gif() -> Result<Html<String>, impl IntoResponse> {
    event!(Level::DEBUG, "Client connected to `/random_cat_gif`");

    let tenor_results = match random_cat_gif_query().await {
        Ok(home_html) => home_html,
        Err(err) => return Err(err),
    };

    let random_cat_gif_html = match get_random_cat_gif_html(tenor_results).await {
        Ok(home_html) => home_html,
        Err(err) => return Err(err),
    };

    Ok(random_cat_gif_html)
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
