use serde::{Deserialize, Serialize};

// -------------------------------------------------------------
// WEBSOCKET MESSAGES
// -------------------------------------------------------------

/// Messages sent from the client to the server.
#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum ClientMessage {
    Echo(EchoPayload),
    GameRoom(GameRoomPayload),
    Chat(ChatPayload),
    TicTacToe(TicTacToePayloadToServer),
}

/// Messages sent from the server to the client.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum ServerMessage {
    Echo(EchoPayload),
    GameRoom(GameRoomPayload),
    Chat(ChatPayload),
    TicTacToe(TicTacToePayloadToClient),
}

// -------------------------------------------------------------
// Payload Structs
// -------------------------------------------------------------

/// Payload for Echo message type
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct EchoPayload {
    pub message: String,
}

/// Payload for GameRoom message type
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GameRoomPayload {
    pub game: String, // "tictactoe", etc
    pub action: String, // "join", "leave", "reset"
    pub player_name: String,
    pub game_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub players: Option<Vec<String>>,
}

/// Payload for Chat message type
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ChatPayload {
    pub game_id: String,
    pub player_name: String,
    pub chat_message: String,
    pub time: String,
}

/// ✅ FIXED: Payload sent TO the client (board as numbers, optional fields)
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TicTacToePayloadToClient {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub board: Option<Vec<Vec<i32>>>, // ✅ Changed from Vec<String> to Vec<Vec<i32>>
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whos_turn: Option<String>, // ✅ Now uses player name instead of "x"/"o"
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>, // "IN_PROGRESS", "gameover_x", "gameover_o", "gameover_draw"
}

/// ✅ FIXED: Payload received FROM the client
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TicTacToePayloadToServer {
    pub game_id: String, // ✅ Required field
    pub whos_turn: String, // Player name making the move
    pub choice: String, // "A1", "B2", etc.
}