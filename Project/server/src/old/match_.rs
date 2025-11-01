use serde_json::Value; 
use crate::game_models::GameLogic; 
use crate::types::{Player, ServerMsg, ClientMsg, ServerMsgBody};
use crate::game_models::tictactoe::TicTacToeState; 

use tokio::sync::mpsc::{self, UnboundedSender};
use std::collections::HashMap;
use std::sync::Arc;
use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Json}; 
use uuid::Uuid; 

// A container for all players in a match. Maps player_id to the Player struct.
pub type PlayerMap = HashMap<String, Player>;
// Type alias for the channel used by the match to send messages out to the client's WebSocket task.
pub type MatchSender = UnboundedSender<String>;

/// The main structure holding a single running match instance.
pub struct Match {
    pub match_id: String,
    pub players: PlayerMap,
    pub game: Box<dyn GameLogic>,
    pub match_tx: MatchSender, // The single channel used to broadcast messages from the match
}

impl Match {
    /// Creates a new, unique Match instance.
    pub fn new(
        match_id: String,
        game: Box<dyn GameLogic>,
        match_tx: MatchSender,
    ) -> Self {
        Match {
            match_id,
            players: HashMap::new(),
            game,
            match_tx,
        }
    }

    /// Sends a message string to the match's internal broadcast channel.
    fn broadcast(&self, message: ServerMsg) {
        if let Ok(json_string) = serde_json::to_string(&message) {
            if let Err(e) = self.match_tx.send(json_string) {
                println!("Error broadcasting message for match {}: {}", self.match_id, e);
            }
        }
    }

    /// Handles a new player joining the match.
    pub fn add_player(&mut self, player_id: String, display_name: String) -> Player {
        // 1. Ask the specific game to assign a role/mark (X or O).
        let new_player = self.game.assign_player(player_id.clone(), display_name.clone());

        // 2. Add the player to the match list.
        self.players.insert(player_id.clone(), new_player.clone());
        
        println!("Player {} ({}) added to match {}", display_name, player_id, self.match_id);

        new_player // Return the newly created Player struct
    }
    
    /// Handles an incoming message from a client, routing it to the game logic.
    pub async fn handle_client_message(&mut self, player_id: &str, msg: ClientMsg) {
        // Only interested in MakeMove messages
        if let ClientMsg::MakeMove { data } = msg {
            // 1. Execute the move via the generic GameLogic trait.
            let result = self.game.make_move(player_id, data);
            
            // 2. Check the result and respond accordingly.
            match result {
                Ok(_) => {
                    // Move succeeded. Broadcast the new game state to all players.
                    let new_state = self.game.get_state(&self.match_id);
                    self.broadcast(new_state);
                }
                Err(e) => {
                    // Move failed validation. Send an error message back to the sender only.
                    let _error_msg = ServerMsg::Error {
                        code: "INVALID_MOVE".to_string(),
                        message: e.clone(), // FIX E0382: Clone 'e' before moving it into the struct
                    };
                    
                    println!("Invalid move by {}: {}", player_id, e);
                    
                    // Note: A complete system would send the error ONLY to the client that failed.
                }
            }
        }
    }

    /// Removes a player from the match.
    pub fn remove_player(&mut self, player_id: &str) {
        self.players.remove(player_id);
        println!("Player {} removed from match {}.", player_id, self.match_id);
    }
}

// --- Match Creation HTTP Handler ---

// Define MatchRegistry type alias for use in the handler
type MatchRegistry = Arc<tokio::sync::Mutex<HashMap<String, Match>>>;

/// HTTP POST handler to create a new match of a specified type.
pub async fn create_match_handler(
    State(registry): State<MatchRegistry>,
    Path((game_type, player_id)): Path<(String, String)>,
) -> impl IntoResponse {
    if game_type != "tictactoe" {
        let msg = format!("Unknown game type: {}", game_type);
        return (StatusCode::BAD_REQUEST, Json(msg));
    }

    // 1. Generate a unique ID for the new match
    let new_match_id = Uuid::new_v4().to_string(); // FIX: Requires 'v4' feature
    
    // 2. Create the game logic instance (TicTacToe in this case)
    let new_game_state = TicTacToeState::new(player_id.clone());
    let generic_game_box: Box<dyn GameLogic> = Box::new(new_game_state);

    // 3. Create the Match container
    let (tx, _rx) = mpsc::unbounded_channel();
    let new_match = Match::new(
        new_match_id.clone(),
        generic_game_box,
        tx,
    );
    
    // 4. Register the new match
    let mut registry_lock = registry.lock().await;
    registry_lock.insert(new_match_id.clone(), new_match);
    
    println!("Created new match: {} of type {}", new_match_id, game_type);

    // 5. Respond with the Match ID so the client can connect via WebSocket
    (StatusCode::CREATED, Json(new_match_id))
}
