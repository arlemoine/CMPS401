// --- Imports ---

use crate::tictactoeModel::{TicTacToeGame, GameMode, Player, GameWinner};
use std::io::{self, Write};
use std::{thread, time::Duration};

// --- Struct ---

/// Class for controller.
pub struct Controller {
    pub name: String,
    pub scores: [u32; 2],
    pub controller_running: bool,
    pub game_mode: GameMode,
}

// --- Implementation ---

impl Controller {
    /// Init controller.
    pub fn new() -> Self {
        Controller {
            name: String::from("Rust TTT Controller"),
            scores: [0, 0],
            controller_running: true,
            game_mode: GameMode::PvAI, // Default to PvAI
        }
    }

    /// Handles the terminal menu selection.
    /// Returns true if the user starts a game, false if they quit.
    pub fn run_menu(&mut self) -> bool {
        loop {
            println!("\n--- TIC-TAC-TOE MENU ---");
            println!("1. Player vs AI");
            println!("2. Player vs Player");
            println!("3. Quit");
            print!("Select an option (1, 2, or 3): ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let choice = input.trim();

            match choice {
                "1" => {
                    self.game_mode = GameMode::PvAI;
                    return true;
                }
                "2" => {
                    self.game_mode = GameMode::PvP;
                    return true;
                }
                "3" => {
                    self.controller_running = false;
                    return false;
                }
                _ => {
                    println!("\nInvalid choice. Please enter 1, 2, or 3.");
                    thread::sleep(Duration::from_millis(500));
                }
            }
        }
    }

    /// Helper to convert terminal input (e.g., "0" to "8") into (row, col) indices.
    /// Returns (row, col) or None if conversion fails.
    fn cli_spot_to_index(spot: &str) -> Option<(usize, usize)> {
        if let Ok(num) = spot.parse::<usize>() {
            if num <= 8 {
                let row = num / 3;
                let col = num % 3;
                return Some((row, col));
            }
        }
        None
    }
    
    /// Draws the game board and status to the terminal. (CLI Only)
    fn draw_board(&self, game_model: &TicTacToeGame) {
        // Clear screen (works in most terminals)
        print!("\x1B[2J\x1B[1;1H"); 
        
        println!("\n--- TIC-TAC-TOE ---");
        println!("SCORES | P1 (X): {} | P2 (O): {}", self.scores[0], self.scores[1]);
        println!("MODE   | {:?}", game_model.game_mode);
        println!("-------------------");
        
        // Draw the 3x3 board
        for r in 0..3 {
            let mut row_display = String::new();
            for c in 0..3 {
                let marker = match game_model.board[r][c] {
                    1 => "X",
                    -1 => "O",
                    _ => &format!("{}", r * 3 + c), // Display the spot number
                };
                row_display.push_str(marker);
                if c < 2 {
                    row_display.push_str(" | ");
                }
            }
            println!("{}", row_display);
            if r < 2 {
                println!("--+---+--");
            }
        }
        println!("-------------------");

        // Display game status
        match game_model.game_state {
            crate::tictactoeModel::GameState::Playing => {
                let player = match game_model.whos_turn {
                    Player::Player1 => "Player 1 (X)",
                    Player::Player2 => "Player 2 (O)",
                };
                println!("It's {}'s turn. Enter spot number (0-8):", player);
            },
            crate::tictactoeModel::GameState::GameOver => {
                match game_model.winner {
                    GameWinner::Player1 => println!("Player 1 (X) WINS!"),
                    GameWinner::Player2 => println!("Player 2 (O) WINS!"),
                    GameWinner::Tie => println!("The game is a TIE!"),
                    _ => println!("Game Over!"),
                }
            },
            _ => { /* Menu is handled in run_menu */ }
        }
    }
    
    /// Allow current player to take turn.
    /// This is a reusable core function.
    pub fn take_turn(&mut self, game_model: &mut TicTacToeGame, row: usize, col: usize) {
        if game_model.validate_choice(row, col) {
            // mark_spot increments turn_counter and updates sums
            game_model.mark_spot(row, col);
            
            // win_check updates game_state and winner if the game ends
            let game_status = game_model.win_check(row, col);

            // Only switch turn if the game is still PLAYING (status == 0)
            if game_status == 0 { 
                game_model.next_turn();
            }
        } else {
            // NOTE: Error handling for invalid moves should be handled by the caller (CLI or GUI)
            // For CLI, we print and sleep, but a GUI would update a message box.
            println!("Invalid move. Spot is taken or out of bounds.");
            thread::sleep(Duration::from_millis(1000));
        }
    }

    /// Gets input from the user (or AI) and processes the move. (CLI Only)
    fn handle_turn(&mut self, game_model: &mut TicTacToeGame) {
        // Initialize mutable variables to hold the coordinates
        let mut row: usize = 0;
        let mut col: usize = 0;

        match game_model.whos_turn {
            Player::Player1 => {
                // Human input loop for Player 1
                loop {
                    self.draw_board(game_model);
                    
                    let mut input = String::new();
                    if io::stdin().read_line(&mut input).is_err() { continue; }

                    if let Some((r, c)) = Controller::cli_spot_to_index(input.trim()) {
                        if game_model.validate_choice(r, c) {
                            row = r;
                            col = c;
                            break; // Exit the loop when a valid move is found
                        } else {
                            // Validation failed, continue loop to prompt again
                            println!("Invalid move. Spot is taken or out of bounds.");
                            thread::sleep(Duration::from_millis(1000));
                        }
                    } else {
                        // Invalid spot format, continue loop
                            println!("Invalid input format. Enter a single number 0-8.");
                            thread::sleep(Duration::from_millis(1000));
                        }
                    }
                }
            },
            Player::Player2 => {
                if game_model.game_mode == GameMode::PvAI {
                    // AI Move (Player 2 is AI)
                    println!("AI is thinking...");
                    thread::sleep(Duration::from_millis(1500)); // Simulate thinking time
                    // Assign coordinates from ai_move()
                    let (r, c) = game_model.ai_move(); 
                    row = r;
                    col = c;
                } else {
                    // Player 2 Human input
                    loop {
                        self.draw_board(game_model);
                        
                        let mut input = String::new();
                        if io::stdin().read_line(&mut input).is_err() { continue; }
    
                        if let Some((r, c)) = Controller::cli_spot_to_index(input.trim()) {
                            if game_model.validate_choice(r, c) {
                                row = r;
                                col = c;
                                break; // Exit the loop when a valid move is found
                            } else {
                                // Validation failed, continue loop to prompt again
                                println!("Invalid move. Spot is taken or out of bounds.");
                                thread::sleep(Duration::from_millis(1000));
                            }
                        } else {
                            // Invalid spot format, continue loop
                            println!("Invalid input format. Enter a single number 0-8.");
                            thread::sleep(Duration::from_millis(1000));
                        }
                    }
                }
            }
        } // The match expression is now purely for control flow, not assignment.
        
        // Execute the move using the coordinates set inside the match blocks
        self.take_turn(game_model, row, col);
    }

    /// Handles the prompt after a game ends, giving the user a chance to quit. (CLI Only)
    fn handle_post_game_menu(&mut self) {
        loop {
            print!("\nPlay another round (Y) or Quit (N)? ");
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let choice = input.trim().to_uppercase();

            match choice.as_str() {
                "Y" => break, // Exit the post-game loop and return to run_cli_loop (play again).
                "N" => {
                    self.controller_running = false;
                    break; // Exit the post-game loop, which terminates run_cli_loop.
                }
                _ => {
                    println!("\nInvalid choice. Please enter Y or N.");
                    thread::sleep(Duration::from_millis(500));
                }
            }
        }
    }
    
    /// Initializes a new TicTacToe game model with the current game mode.
    /// This is the CORE, reusable initialization logic for both CLI and GUI.
    pub fn new_game(&mut self) -> TicTacToeGame {
        // Start a new game using the last selected mode
        let mut game_model = TicTacToeGame::new(self.game_mode);
        
        // Ensure the game starts in the Playing state
        game_model.game_state = crate::tictactoeModel::GameState::Playing; 
        
        // Return the initial game state.
        game_model
    }

    /// Manages the terminal-specific game flow for one round. (CLI Only)
    fn run_cli_game(&mut self) {
        // Create model for this round using the reusable function
        let mut game_model = self.new_game();
        
        // The game loop runs until the game_state is GameState::GameOver
        while game_model.game_state == crate::tictactoeModel::GameState::Playing {
            self.handle_turn(&mut game_model);
        }

        // Game Over screen (CLI specific)
        self.draw_board(&game_model);
        
        // Update scores based on winner
        match game_model.winner {
            GameWinner::Player1 => self.scores[0] += 1,
            GameWinner::Player2 => self.scores[1] += 1,
            _ => {}
        }
    }
        
    /// Start the primary application loop for the Command Line Interface. (CLI Only)
    pub fn run_cli_loop(&mut self) {
        while self.controller_running {
            self.run_cli_game();
            // Now, we ask for user input to continue or quit
            self.handle_post_game_menu();
        }
        println!("\nThank you for playing!");
    }
}
