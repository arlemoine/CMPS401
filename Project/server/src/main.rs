use axum::{routing::get, Router};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use tracing::info;
use tracing_subscriber::EnvFilter;

mod matchmaker;
mod types;
mod ws;

use matchmaker::MatchRegistry;
use ws::ws_upgrade;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse().unwrap()))
        .init();

    let registry: MatchRegistry = Arc::new(RwLock::new(HashMap::new()));

    let app = Router::new()
        .route("/health", get(health))
        .route("/ws", get({
            let registry = Arc::clone(&registry);
            move |ws| ws_upgrade(ws, registry)
        }));

    let listener = TcpListener::bind("127.0.0.1:3001").await.unwrap();
    info!(addr = %listener.local_addr().unwrap(), "server listening");
    axum::serve(listener, app).await.unwrap();
}

async fn health() -> &'static str {
    "OK"
}
