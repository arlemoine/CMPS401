use tokio::sync::RwLock;
use std::collections::HashMap;

use crate::types::gameroom::GameRoom;

// Holds state of the application backend
#[derive(Default)]
pub struct AppState {
    pub rooms: RwLock<HashMap<String, GameRoom>>, // key, value pair (room_id: String, room_object: GameRoom)
}