use crate::types::ServerMsg;
use serde_json::Value;
use tokio::sync::mpsc::UnboundedSender;
use std::collections::HashMap;

// Type alias for the channel map (player_id -> sender)
pub type ListenerMap = HashMap<String, UnboundedSender<String>>;

pub trait GameLogic: Send + Sync {
    /// Returns the current state packaged as a ServerMsg.
    fn get_state(&self, match_id: &str) -> ServerMsg;

    /// Attempts to execute a move based on the provided JSON data.
    fn make_move(&mut self, player_id: &str, data: Value) -> Result<(), String>;

    /// Assigns a player to a role (e.g., X or O) and returns the Player struct.
    fn assign_player(&mut self, player_id: String, display_name: String) -> crate::types::Player;
    
    /// Provides mutable access to the list of listening client channels.
    fn get_listeners(&mut self) -> &mut ListenerMap; // <-- FIX: ADDED THIS METHOD

    fn is_finished(&self) -> bool;
}

// Note: You must also implement this new method in your TicTacToeState struct.

pub mod tictactoe;
