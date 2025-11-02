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
    Chat (ChatPayload),
    TicTacToe (TicTacToePayloadToServer),
}

/// Messages sent from the server to the client.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum ServerMessage { 
    Echo (EchoPayload),
    GameRoom (GameRoomPayload),
    Chat (ChatPayload),
    TicTacToe (TicTacToePayloadToClient),
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

/// Payload for GameRoom message type
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ChatPayload {
    pub action: String, // "join" or "leave"
    pub game_id: String,
    pub player_name: String,
    pub chat_message: String,
}

/// Payload for TicTacToe game operations
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TicTacToePayloadToClient {
    pub board: Vec<String>,
    pub whos_turn: String, // "x", "o"
    pub status: String, // "next_x", "next_o", "gameover_tie", "gameover_x", "gameover_o"
}

/// Payload for TicTacToe game operations
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TicTacToePayloadToServer {
    pub whos_turn: String, // "x", "o"
    pub choice: String, // "A1"
}
