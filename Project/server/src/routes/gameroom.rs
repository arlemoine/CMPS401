use std::sync::Arc;
use tokio::sync::{
    broadcast,
    RwLock,
};
use axum::extract::ws::Message;

#[derive(Clone, Default)]
pub struct GameState {
    pub game: String, // e.g. "chess", "checkers"
    pub state_data: serde_json::Value, // flexible per game type
}

#[derive(Clone, Default)]
pub struct GameRoom {
    pub users: Arc<RwLock<Vec<String>>>,
    pub chat_log: Arc<RwLock<Vec<String>>>,
    pub game_state: Arc<RwLock<GameState>>,
    // pub tx: broadcast::Sender<String>,  // per-room broadcast channel
}
