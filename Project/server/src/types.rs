use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::models::uno::model::UnoCard;

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
    RockPaperScissors(RockPaperScissorsPayloadToServer),
    Uno(UnoPayloadToServer),
    AirHockey(AirHockeyPayloadToServer),
}

/// Messages sent from the server to the client.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum ServerMessage {
    Echo(EchoPayload),
    GameRoom(GameRoomPayload),
    Chat(ChatPayload),
    TicTacToe(TicTacToePayloadToClient),
    RockPaperScissors(RockPaperScissorsPayloadToClient),
    Uno(UnoPayloadToClient),
    AirHockey(AirHockeyPayloadToClient),
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
    pub game: String, // "tictactoe", "rockpaperscissors", etc
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

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TicTacToePayloadToClient {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub board: Option<Vec<Vec<i32>>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whos_turn: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TicTacToePayloadToServer {
    pub game_id: String,
    pub whos_turn: String, // Player name making the move
    pub choice: String, // "A1", "B2", etc.
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RockPaperScissorsPayloadToServer {
    pub game_id: String,
    pub player_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub choice: Option<String>, // optional so clients may request latest state
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RockPaperScissorsPayloadToClient {
    pub game_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player1_choice: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player2_choice: Option<String>,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub winner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct UnoPayloadToServer {
    pub game_id: String, /// Game session identifier
    pub player_name: String, /// Player sending the request
    pub action: String, /// Action verb: "start", "play_card", "draw_card", "pass_turn", "call_uno", "request_state"
    
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub card: Option<UnoCard>, /// Optional card included when action == "play_card". UnoCard = { color: String, rank: String }

    #[serde(skip_serializing_if = "Option::is_none")]
    pub choose_color: Option<String>, /// Required when playing Wild / WildDrawFour; one of "Red","Yellow","Green","Blue" (non-binding UI hint only; does NOT lock color)

    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_uno: Option<bool>, // If the client presses UNO at the moment they go to 1 card
}


/// Payload sent TO the client for Uno
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct UnoPayloadToClient {
    pub game_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub players: Option<Vec<String>>, /// Seat order of players (public names). Index corresponds to turn order.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_idx: Option<i32>, /// Current turn index into `players`

    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<i8>, /// Direction of play clockwise or counterclockwise (1 or -1) 

    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_discard: Option<UnoCard>, /// Top card on the discard pile

    #[serde(skip_serializing_if = "Option::is_none")]
    pub chosen_color: Option<String>, /// Non-binding color hint chosen on Wild/WDF; does NOT restrict rank plays

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_draw: Option<u8>, /// If someone owes a draw due to DrawTwo/WildDrawFour

    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_counts: Option<Vec<u8>>,  /// Public info: hand sizes in seat order (no card identities)

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hand: Option<Vec<UnoCard>>, /// Private hand for the receiving client only

    #[serde(skip_serializing_if = "Option::is_none")]
    pub winner: Option<String>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct AirHockeyPayloadToServer {
    pub action: String, // "move_paddle", "request_state"
    pub game_id: String,
    pub player_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Vector2>, // { x: f32, y: f32 }
    #[serde(skip_serializing_if = "Option::is_none")]
    pub velocity: Option<Vector2>, // { x: f32, y: f32 }
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<f64>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Serialize, Debug)]
pub struct AirHockeyPayloadToClient {
    pub event: String, // "update", "p1_score", "p2_score", "game_over_winner_p1", "game_over_winner_p2"
    pub game_id: String,
    pub timestamp: f64, /// Current timestamp for interpolation / latency compensation
    pub paddles: HashMap<String, PaddleState>, // player_id => state
    pub puck: PuckState,
    pub score: HashMap<String, u8>, // player_id => score
}

#[derive(Clone, Serialize, Debug)]
pub struct PaddleState {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
}

#[derive(Clone, Serialize, Debug)]
pub struct PuckState {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
}