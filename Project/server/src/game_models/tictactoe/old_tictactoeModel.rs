use rand::seq::SliceRandom; // Used for AI move selection
use rand::prelude::IndexedRandom; 


// --- ENUMS (Corresponds to Python's GameState, GameMode, GameWinner) ---

/// Related enum for the overall state of the game.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    Menu,
    Playing,
    GameOver,
}

/// Related enum for the mode of play.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameMode {
    PvAI,   // Player vs AI
    PvP,    // Player vs Player
}

/// Defines the two players and their corresponding mark values for the board.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    Player1, // Corresponds to value 1 (X)
    Player2, // Corresponds to value -1 (O)
}

impl Player {
    /// Returns the integer value used for board marking and win condition summation.
    pub fn mark_value(&self) -> i8 {
        match self {
            Player::Player1 => 1,
            Player::Player2 => -1,
        }
    }
}

/// Related enum for the game winner status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameWinner {
    Pending,
    Player1,
    Player2,
    Tie,
}

// --- STRUCT (Corresponds to Python's TicTacToeGame class) ---

/// The core model for a game of Tic-Tac-Toe.
pub struct TicTacToeGame {
    // General State
    pub game_running: bool,
    pub game_state: GameState,
    pub game_mode: GameMode,
    pub winner: GameWinner,
    pub whos_turn: Player,
    pub turn_counter: u8,
    
    // Board status: i8 is used here to match the Python logic (1, -1, 0)
    pub board: [[i8; 3]; 3],
    pub row_sum: [i8; 3],
    pub col_sum: [i8; 3],
    pub diag_sum: [i8; 2],
}

// --- IMPLEMENTATION (Corresponds to Python's class methods) ---

impl TicTacToeGame {
    /// Constructor: Initializes a new game instance.
    pub fn new(game_mode: GameMode) -> Self {
        TicTacToeGame {
            game_running: true,
            game_state: GameState::Menu,
            game_mode,
            winner: GameWinner::Pending,
            whos_turn: Player::Player1, // Player 1 (X) starts
            turn_counter: 0,
            
            // Initialization for board state
            board: [[0; 3]; 3],
            row_sum: [0; 3],
            col_sum: [0; 3],
            diag_sum: [0; 2],
        }
    }

    /// Validates the choice of row and column selection.
    /// Returns true if valid, or false if invalid.
    pub fn validate_choice(&self, row: usize, col: usize) -> bool {
        // 1. Check bounds
        if row >= 3 || col >= 3 {
            // Log error
            return false; 
        }
        
        // 2. Check if spot is empty (board value must be 0)
        if self.board[row][col] != 0 {
            // Log error
            return false;
        }
        
        true // If all checks pass, return true
    }

    /// Marks a spot, increments sums for win checks, and advances turn count.
    pub fn mark_spot(&mut self, row: usize, col: usize) {
        // Get the value (1 or -1) of the current player
        let mark_value = self.whos_turn.mark_value();

        // Mark spot on the board
        self.board[row][col] = mark_value;
        
        // Increment sums related to victory conditions
        self.row_sum[row] += mark_value;
        self.col_sum[col] += mark_value;

        // Check diagonals
        if row == col {
            self.diag_sum[0] += mark_value;
        }
        if row + col == 2 {
            self.diag_sum[1] += mark_value;
        }
        
        // Advance turn counter
        self.turn_counter += 1;
    }

    /// Determines if a winner is found based on the latest move.
    /// Returns the status: 0=Continue, 1=Win, 2=Tie.
    pub fn win_check(&mut self, row: usize, col: usize) -> u8 {
        // Win value is 3 or -3, based on the current player's mark value
        let win_value = self.whos_turn.mark_value() * 3;
        let potential_winner = match self.whos_turn {
            Player::Player1 => GameWinner::Player1,
            Player::Player2 => GameWinner::Player2,
        };
        
        let mut status = 0;
        
        // 1. Check for Win (Row, Col, Diagonals)
        if self.row_sum[row] == win_value || self.col_sum[col] == win_value {
            status = 1;
        }
        if row == col && self.diag_sum[0] == win_value {
            status = 1;
        }
        if row + col == 2 && self.diag_sum[1] == win_value {
            status = 1;
        }

        if status == 1 {
            self.winner = potential_winner;
            self.game_state = GameState::GameOver;
        } 
        
        // 2. Check for Tie Game
        else if self.turn_counter == 9 {
            self.winner = GameWinner::Tie;
            self.game_state = GameState::GameOver;
            status = 2;
        }

        status
    }
            
    /// Switches to the next player's turn.
    pub fn next_turn(&mut self) {
        self.whos_turn = match self.whos_turn {
            Player::Player1 => Player::Player2,
            Player::Player2 => Player::Player1,
        };
    }

    /// Chooses a spot for the AI player.
    /// Returns (row, col) indices.
    pub fn ai_move(&self) -> (usize, usize) {
        // Obtain list of available spots
        let mut free_spots = Vec::new();
        for i in 0..3 {
            for j in 0..3 {
                if self.board[i][j] == 0 {
                    free_spots.push((i, j));
                }
            }
        }

        // 1. Check for immediate win/defense targets
        let (focus_direction, focus_index) = self.ai_check();

        if !focus_direction.is_empty() {
            // Target the winning/defending spot
            for spot in &free_spots {
                let (r, c) = *spot;
                match focus_direction.as_str() {
                    "row" => if r == focus_index { return (r, c) },
                    "col" => if c == focus_index { return (r, c) },
                    "diag" => {
                        if focus_index == 0 && r == c { return (r, c) }
                        if focus_index == 1 && r + c == 2 { return (r, c) }
                    },
                    _ => {} // Should not happen
                }
            }
        }
        
        // 2. Default: Choose a random available spot
        // unwrap is safe because free_spots will only be empty if game_over == true
        *free_spots.choose(&mut rand::rng()).unwrap()
    }

    /// Determines if the AI should target a particular spot to win or defend.
    /// Returns (focus_direction, focus_index).
    pub fn ai_check(&self) -> (String, usize) {
        // Values we are looking for: 
        // Offense (AI is O, so -1 * 2 = -2) -> Win next turn
        // Defense (Opponent is X, so 1 * 2 = 2) -> Block opponent's win
        let (offense_val, defense_val) = (-2, 2);

        let mut focus_index_offense = 9;
        let mut focus_direction_offense = String::new();
        let mut focus_index_defense = 9;
        let mut focus_direction_defense = String::new();

        // Rows
        for i in 0..3 {
            if self.row_sum[i] == offense_val {
                focus_index_offense = i;
                focus_direction_offense = "row".to_string();
            } else if self.row_sum[i] == defense_val {
                focus_index_defense = i;
                focus_direction_defense = "row".to_string();
            }
        }
        
        // Cols
        for i in 0..3 {
            if self.col_sum[i] == offense_val {
                focus_index_offense = i;
                focus_direction_offense = "col".to_string();
            } else if self.col_sum[i] == defense_val {
                focus_index_defense = i;
                focus_direction_defense = "col".to_string();
            }
        }
        
        // Diagonals (Index 0: Top-Left to Bottom-Right, Index 1: Top-Right to Bottom-Left)
        for i in 0..2 {
            if self.diag_sum[i] == offense_val {
                focus_index_offense = i;
                focus_direction_offense = "diag".to_string();
            } else if self.diag_sum[i] == defense_val {
                focus_index_defense = i;
                focus_direction_defense = "diag".to_string();
            }
        }

        // Prioritize Offense (AI win) over Defense (Block opponent)
        if !focus_direction_offense.is_empty() {
            (focus_direction_offense, focus_index_offense)
        } else if !focus_direction_defense.is_empty() {
            (focus_direction_defense, focus_index_defense)
        } else {
            // No immediate win or block needed
            (String::new(), 9)
        }
    }
}
