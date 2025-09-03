use axum::{Router, http::StatusCode, routing::get};
use serde::Deserialize;
use std::net::{Ipv4Addr, SocketAddr};
use tokio::signal;

fn default_port() -> u16 {
    3000
}

#[derive(Debug, Deserialize)]
struct ServerConfig {
    #[serde(default = "default_port")]
    port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            port: default_port(),
        }
    }
}

#[derive(Debug, Deserialize, Default)]
struct AppConfig {
    #[serde(default)]
    server: ServerConfig,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let settings = config::Config::builder()
        .add_source(config::File::with_name("config").required(false))
        .build()?;

    let cfg: AppConfig = settings.try_deserialize().unwrap_or_default();
    tracing::info!("Loaded config: {:?}", cfg);

    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, cfg.server.port));

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health", get(StatusCode::OK));

    let listener = tokio::net::TcpListener::bind(addr).await?;

    tracing::info!("listening on http://{}", addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            let _ = signal::ctrl_c().await;
        })
        .await?;

    Ok(())
}
