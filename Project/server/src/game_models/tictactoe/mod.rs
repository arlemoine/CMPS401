use serde_json::Value;
use crate::game_models::GameLogic;
use crate::types::{ServerMsg, Player as PlatformPlayer};
use crate::game_models::ListenerMap;
use std::collections::HashMap;

// --- ENUMS (Specific to Tic-Tac-Toe Logic) ---

/// Related enum for the overall state of the game.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    Playing,
    GameOver,
}

/// Defines the two players and their corresponding mark values for the board.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerMark {
    X, // Corresponds to value 1
    O, // Corresponds to value -1
}

impl PlayerMark {
    /// Returns the integer value used for board marking and win condition summation.
    pub fn mark_value(&self) -> i8 {
        match self {
            PlayerMark::X => 1,
            PlayerMark::O => -1,
        }
    }
    /// Returns the string representation for network messages.
    pub fn to_string(&self) -> String {
        match self {
            PlayerMark::X => "X".to_string(),
            PlayerMark::O => "O".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameWinner {
    Pending,
    X,
    O,
    Tie,
}

// --- Core data model ---

/// The core state for a game of Tic-Tac-Toe.
pub struct TicTacToeState {
    // General State
    pub game_state: GameState,
    pub winner: GameWinner,
    pub whos_turn: PlayerMark,
    pub turn_counter: u8,
    
    // Board status (using 3x3 array to match original user logic)
    pub board: [[i8; 3]; 3],
    pub row_sum: [i8; 3],
    pub col_sum: [i8; 3],
    pub diag_sum: [i8; 2],

    // Player mapping (Maps mark to the platform's Player ID)
    player_x_id: String,
    player_o_id: Option<String>,
    pub listeners: ListenerMap, // 👈 Add this
}

impl TicTacToeState {
    /// Constructor: Initializes a new game instance.
    pub fn new(player_x_id: String) -> Self {
        TicTacToeState {
            game_state: GameState::Playing,
            winner: GameWinner::Pending,
            whos_turn: PlayerMark::X,
            turn_counter: 0,
            
            // Initialization for board state
            board: [[0; 3]; 3],
            row_sum: [0; 3],
            col_sum: [0; 3],
            diag_sum: [0; 2],

            player_x_id,
            player_o_id: None,
            listeners: HashMap::new(), // 👈 new field init
        }
    }
}

// --- IMPLEMENTATION OF THE SHARED PLATFORM INTERFACE ---

impl GameLogic for TicTacToeState {
    /// Handles the move logic for any game type.
    fn make_move(&mut self, player_id: &str, move_data: Value) -> Result<(), String> {
        // Placeholder for the actual move logic (will be implemented next)
        if self.game_state == GameState::GameOver {
            return Err("Game is already over.".to_string());
        }
        
        // This line is a temporary action to satisfy the trait
        println!("Placeholder: Move received from {} with data: {:?}", player_id, move_data);
        Ok(())
    }

    /// Generates the current state snapshot for broadcasting.
    fn get_state(&self, match_id: &str) -> ServerMsg {
        // Placeholder for generating the ServerMsg (will be implemented next)
        ServerMsg::Error {
            code: "NOT_IMPLEMENTED".to_string(),
            message: format!("State for match {} not implemented yet.", match_id),
        }
    }
    
    /// Assigns the X or O mark to a player when they join.
    fn assign_player(&mut self, id: String, display_name: String) -> PlatformPlayer {
        let mark = if self.player_o_id.is_none() {
            self.player_o_id = Some(id.clone());
            PlayerMark::O
        } else {
            PlayerMark::X 
        };
        
        PlatformPlayer {
            id,
            display_name,
            mark: mark.to_string(),
        }
    }

    /// Checks if the game is finished.
    fn is_finished(&self) -> bool {
        self.game_state == GameState::GameOver
    }

    fn get_listeners(&mut self) -> &mut ListenerMap {
        &mut self.listeners
    }
}
