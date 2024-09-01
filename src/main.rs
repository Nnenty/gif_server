use axum::{routing::get, Router};
use handlers::{home, random_cat_gif, search_gif};
use tokio::{self, signal};

use anyhow;
use tracing::debug;
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

pub mod handlers;
pub mod presentation;
pub mod tenor;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    let config_toml_path = "config.toml";
    let config = std::fs::read_to_string(config_toml_path)?;

    let Config { server, trace } = toml::from_str(&config)?;

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(
            EnvFilter::new(trace.logging_level)
                .add_directive("reqwest=warn".parse().expect("invalid directive"))
                .add_directive("hyper=warn".parse().expect("invalid directive"))
                .add_directive("globset=warn".parse().expect("invalid directive")),
        )
        .init();

    let (host, port) = (server.host, server.port);

    debug!("PORT: {port}, HOST: {host}");

    let app = Router::new()
        .route("/", get(home))
        .route("/random_cat_gif", get(random_cat_gif))
        .route("/search_gif", get(search_gif));
    let listener = tokio::net::TcpListener::bind(format!("{host}:{port}")).await?;

    debug!("Server start listening");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
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
        () = ctrl_c => {},
        () = terminate => {},
    }
}
