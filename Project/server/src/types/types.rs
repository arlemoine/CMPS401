use serde::{Deserialize, Serialize};

// -------------------------------------------------------------
// WEBSOCKET MESSAGES
// -------------------------------------------------------------

/// Messages sent from the client to the server.
#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum ClientMessage {
    Echo (EchoPayload),
    GameRoom (GameRoomPayload),
    // TicTacToe (TicTacToePayload),
}

/// Messages sent from the server to the client.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum ServerMessage { 
    Echo (EchoPayload),
    GameRoom (GameRoomPayload),
    // TicTacToe (TicTacToePayload),
}

// -------------------------------------------------------------
// Payload Structs
// -------------------------------------------------------------

/// Payload for GameRoom message type
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct EchoPayload {
    pub message: String,
}

/// Payload for GameRoom message type
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GameRoomPayload {
    pub action: String, // "join" or "leave"
    pub player_name: String,
    pub game_id: String,
}

// /// Payload for TicTacToe game operations
// #[derive(Clone, Serialize, Deserialize, Debug)]
// pub struct TicTacToePayload {
//     pub board: Vec<String>,
//     pub turn: String,
//     pub status: String,
//     pub winner: String,
//     pub player_x_id: String,
//     pub player_o_id: Option<String>,
// }
