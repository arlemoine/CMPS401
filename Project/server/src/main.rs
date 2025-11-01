<<<<<<< HEAD
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
=======
// NOTE: This is currently only a skeleton which opens a websocket connection to the frontend and echoes messages received from the frontend back to the frontend.
use axum::{
    routing::{get}, 
    Router,
};
use std::{
    net::SocketAddr,
    sync::Arc,
};
use tokio;
use tracing_subscriber;

// Declare project modules
mod types;
mod config;
mod routes;
mod ws;

use types::appstate::AppState;
use config::Config;
use ws::ws_handler;


#[tokio::main]
async fn main() {
    // Pull from config file
    let config = Config::default();

    // Init logger
    tracing_subscriber::fmt()
        .with_max_level(config.log_level)
        .init();

    // Define socket address and utilize logger
    let addr: SocketAddr = config.socket_addr();
    tracing::info!("listening on {}", addr);

    // Init game state
    let state = Arc::new(AppState::default());

    // Init router with routes
    let app = Router::new()
        .route("/ws", get(ws_handler))
        .with_state(state);

    // Init listener
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_default_works() {
        let config = Config::default();
        // Just check that socket address parses
        let addr: SocketAddr = config.socket_addr();
        assert!(addr.ip().is_loopback());
    }

    #[tokio::test]
    async fn state_initialization() {
        let state = Arc::new(AppState::default());
        // Ensure rooms hashmap exists and is empty initially
        assert_eq!(state.rooms.read().await.len(), 0);
    }
>>>>>>> origin/dev
}
