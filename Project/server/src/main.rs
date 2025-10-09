// Declare the modules so the Rust compiler knows to look for them
mod tictactoeModel;
mod tictactoeController;

// Import what we need from the modules
use tictactoeController::Controller;
use tictactoeModel::GameMode;

use std::io::{self, Write};
use std::{thread, time::Duration};

/// The main entry function for the Tic-Tac-Toe terminal application.
fn main() {
    let mut controller = Controller::new();
    
    // Delegate the terminal menu handling to the Controller.
    // The Controller will set the chosen game mode internally.
    // If run_menu returns true, the user wants to play. If false, the user quit.
    if controller.run_menu() {
        // Start the main game loop
        controller.new_game_loop();
    }
}
