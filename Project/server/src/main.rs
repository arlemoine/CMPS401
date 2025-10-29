// NOTE: This is currently only a skeleton which opens a websocket connection to the frontend and echoes messages received from the frontend back to the frontend.
use axum::{
    Extension,
    routing::get, 
    Router,
};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::Arc,
};
use tokio::{
    net::TcpListener,
    sync::RwLock,
};
use tracing_subscriber;

// Declare project modules
mod appstate;
mod config;
mod routes;
mod types;
mod ws;

use config::Config;
use appstate::AppState;
use routes::gameroom;
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
