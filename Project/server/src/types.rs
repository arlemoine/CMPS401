// src/types.rs

use serde::{Deserialize, Serialize};

// The data type for a player connected to a match (Unchanged)
#[derive(Debug, Clone, Serialize)]
pub struct Player {
    pub id: String,
    pub display_name: String,
    pub mark: String, 
}

// -------------------------------------------------------------
// CLIENT TO SERVER MESSAGES (ClientMsg)
// -------------------------------------------------------------

/// Messages sent from the client (web browser) to the server.
#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum ClientMsg {
    /// Request to make a move. The data field contains game-specific move data (e.g., {"row": 1, "col": 2}).
    MakeMove { data: serde_json::Value }, // FIX E0599: Variant must be `MakeMove`
    
    /// Request to chat (optional for later)
    Chat { message: String },
}

// -------------------------------------------------------------
// SERVER TO CLIENT MESSAGES (ServerMsg)
// -------------------------------------------------------------

/// Helper enum to contain game-specific state inside a ServerMsg
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "game_type", content = "state")]
pub enum ServerMsgBody { // FIX E0412: Define ServerMsgBody
    TicTacToe {
        board: Vec<String>,
        turn: String,
        status: String,
        winner: String,
        player_x_id: String,
        player_o_id: Option<String>,
    },
}

/// Messages sent from the server to the client.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum ServerMsg {
    /// Message sent immediately after a successful WebSocket connection.
    JoinedMatch {
        you: Player, // FIX E0559: Field is `you`, not `player`
        match_id: String,
        body: ServerMsgBody,
    },
    
    /// Regular message containing the current game state for all players.
    GameState {
        match_id: String,
        body: ServerMsgBody,
    },
    
    /// Error message sent if client move validation fails.
    Error {
        code: String,
        message: String,
    },

    Chat {
        sender: String,
        message: String,
    }
}

// Helper to extract the body for the initial state send
impl ServerMsg {
    pub fn get_body(&self) -> &ServerMsgBody { // FIX E0599: Define get_body
        match self {
            ServerMsg::JoinedMatch { body, .. } => body,
            ServerMsg::GameState { body, .. } => body,
            // You may need to adjust this panic based on your needs
            _ => panic!("Attempted to get body from a non-state message"),
        }
    }
}