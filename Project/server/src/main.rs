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
mod config;
mod models;
mod routes;
mod types;
mod ws;

use models::appstate::AppState;
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
}
