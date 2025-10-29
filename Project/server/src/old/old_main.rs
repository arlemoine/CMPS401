use axum::{
    routing::{get, post},
    extract::{WebSocketUpgrade, State},
    Router,
};
use std::collections::HashMap; // 👈 FIX E0433: Import HashMap
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::Mutex; // 👈 We use Mutex, not RwLock in this example
use uuid::Uuid;

// Global Application State (MatchRegistry)
// Holds all active matches, protected by a Mutex for safe concurrent access.
type MatchRegistry = Arc<Mutex<HashMap<String, Match>>>; // 👈 Define MatchRegistry here

mod match_; // 👈 Fix E0583: Assumes you renamed file to src/match_.rs
mod types;
mod game_models;
mod ws;

use crate::match_::Match; // 👈 Use the local Match struct

#[tokio::main]
async fn main() {
    // 1. Initialize the global match registry
    let registry: MatchRegistry = Arc::new(Mutex::new(HashMap::new()));

    // 2. Configure the HTTP routes
    let app = Router::new()
        .route("/match/new/{game_type}/{player_id}", post(match_::create_match_handler))
        // Add this new route so the frontend can connect without parameters:
        .route("/ws", get(|ws: WebSocketUpgrade, State(registry): State<MatchRegistry>| async move {
            let match_id = "default_match".to_string();
            let player_id = "guest_player".to_string();
            ws.on_upgrade(|socket| crate::ws::handle_socket(socket, registry, match_id, player_id))
        }))
        .with_state(registry);

    // 3. Start the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001")
        .await
        .unwrap();

    println!("Listening on http://0.0.0.0:3001");

    axum::serve(listener, app).await.unwrap();
}
