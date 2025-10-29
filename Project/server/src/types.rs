use serde::{Deserialize, Serialize};

// -------------------------------------------------------------
// CLIENT TO SERVER MESSAGES (ClientMessage)
// -------------------------------------------------------------

/// Messages sent from the client to the server.
#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum ClientMessage {
    Echo { 
        message: String,
    },
    // JoinRoom { 
    //     room_id: String,
    // },
    // MovePiece {
    //     from: String, 
    //     to: String,
    // },
    // Chat { 
    //     text: String,
    // },
}

// -------------------------------------------------------------
// SERVER TO CLIENT MESSAGES (ServerMessage)
// -------------------------------------------------------------

/// Messages sent from the server to the client.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "game_type", content = "state")]
pub enum ServerMessage { 
    Echo { 
        message: String,
    },
    // Joined {
    //     room_id: String,
    // },
    // TicTacToe {
    //     board: Vec<String>,
    //     turn: String,
    //     status: String,
    //     winner: String,
    //     player_x_id: String,
    //     player_o_id: Option<String>,
    // },
}

// -------------------------------------------------------------
// OTHER
// -------------------------------------------------------------



// /// 
// #[derive(Debug, Clone, Serialize)]
// #[serde(tag = "type")]
// pub enum ServerMsg {
//     /// Message sent immediately after a successful WebSocket connection.
//     JoinedMatch {
//         you: Player, // FIX E0559: Field is `you`, not `player`
//         match_id: String,
//         body: ServerMsgBody,
//     },
    
//     /// Regular message containing the current game state for all players.
//     GameState {
//         match_id: String,
//         body: ServerMsgBody,
//     },
    
//     /// Error message sent if client move validation fails.
//     Error {
//         code: String,
//         message: String,
//     },

//     Chat {
//         sender: String,
//         message: String,
//     }
// }

// // Helper to extract the body for the initial state send
// impl ServerMsg {
//     pub fn get_body(&self) -> &ServerMsgBody { // FIX E0599: Define get_body
//         match self {
//             ServerMsg::JoinedMatch { body, .. } => body,
//             ServerMsg::GameState { body, .. } => body,
//             // You may need to adjust this panic based on your needs
//             _ => panic!("Attempted to get body from a non-state message"),
//         }
//     }
// }

// // The data type for a player connected to a match (Unchanged)
// #[derive(Debug, Clone, Serialize)]
// pub struct Player {
//     pub id: String,
//     pub display_name: String,
//     pub mark: String, 
// }