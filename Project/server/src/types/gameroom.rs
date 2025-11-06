use tokio::sync::mpsc::UnboundedSender;
use axum::extract::ws::Message;

#[derive(Default)]
pub struct GameState {
    pub game: String, // e.g. "chess", "checkers"
    pub state_data: serde_json::Value, // flexible per game type
}

#[derive(Default)]
pub struct GameRoom {
    pub users: Vec<String>,
    pub txs: Vec<UnboundedSender<Message>>, // holds tranceivers for all members of game room to broadcast
    pub chat_log: Vec<String>,
    pub game_state: GameState,
}
