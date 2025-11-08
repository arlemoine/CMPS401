use std::sync::Arc;
use tokio::sync::RwLock;
use crate::types::{ServerMessage, TicTacToePayloadToClient, TicTacToePayloadToServer};
use crate::models::gameroom::{GameRoom, GameType};
use crate::models::tictactoe::model::{TicTacToeModel, Player, GameWinner};
use crate::models::appstate::AppState;

/// Handles incoming TicTacToe messages from clients.
pub async fn tictactoe_handler(
    payload: TicTacToePayloadToServer,
    app_state: &Arc<AppState>,
    current_room: Arc<RwLock<Option<String>>>,
) -> ServerMessage {
    // ✅ FIXED: Use game_id from payload instead of current_room
    let game_id = payload.game_id.clone();
    
    // Get a mutable handle to the rooms map, then the room
    let mut rooms = app_state.rooms.write().await;
    let room = match rooms.get_mut(&game_id) {
        Some(r) => r,
        None => {
            eprintln!("[TicTacToe] Room not found: {}", game_id);
            return ServerMessage::TicTacToe(TicTacToePayloadToClient {
                board: Some(vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]),
                whos_turn: Some("".to_string()),
                status: Some("room_not_found".to_string()),
            });
        }
    };

    // Extract the TicTacToe model from the room
    let game = match &mut room.game {
        GameType::TicTacToe(m) => m,
        _ => {
            eprintln!("Tried to play TicTacToe in a non-TicTacToe room: {}", game_id);
            return ServerMessage::TicTacToe(TicTacToePayloadToClient {
                board: Some(vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]),
                whos_turn: Some("".to_string()),
                status: Some("wrong_game_type".to_string()),
            });
        }
    };

    // ✅ FIXED: Assign players if not yet assigned
    if game.player1_name.is_none() && !room.users.is_empty() {
        game.player1_name = Some(room.users[0].clone());
    }
    if game.player2_name.is_none() && room.users.len() > 1 {
        game.player2_name = Some(room.users[1].clone());
    }

    // Determine which Player this move is from
    let player = match game.get_player_from_name(&payload.whos_turn) {
        Some(p) => p,
        None => {
            eprintln!("[TicTacToe] Unknown player: {}", payload.whos_turn);
            return ServerMessage::TicTacToe(TicTacToePayloadToClient {
                board: Some(serialize_board_as_numbers(game)),
                whos_turn: game.current_player_name().map(|s| s.to_string()),
                status: Some("unknown_player".to_string()),
            });
        }
    };

    // Check if it's actually this player's turn
    if player != game.whos_turn {
        eprintln!("[TicTacToe] Not {}'s turn", payload.whos_turn);
        return ServerMessage::TicTacToe(TicTacToePayloadToClient {
            board: Some(serialize_board_as_numbers(game)),
            whos_turn: game.current_player_name().map(|s| s.to_string()),
            status: Some("not_your_turn".to_string()),
        });
    }

    // Parse choice like "A1"
    let (row, col) = match parse_choice(&payload.choice) {
        Some(rc) => rc,
        None => {
            eprintln!("[TicTacToe] Invalid choice: {}", payload.choice);
            return ServerMessage::TicTacToe(TicTacToePayloadToClient {
                board: Some(serialize_board_as_numbers(game)),
                whos_turn: game.current_player_name().map(|s| s.to_string()),
                status: Some("invalid_choice".to_string()),
            });
        }
    };

    if !game.validate_choice(row, col) {
        eprintln!("[TicTacToe] Invalid move at ({}, {})", row, col);
        return ServerMessage::TicTacToe(TicTacToePayloadToClient {
            board: Some(serialize_board_as_numbers(game)),
            whos_turn: game.current_player_name().map(|s| s.to_string()),
            status: Some("invalid_move".to_string()),
        });
    }

    // Make the move
    game.mark_spot(row, col);
    game.check_winner();
    
    println!("[TicTacToe] {} made move at {}", payload.whos_turn, payload.choice);
    println!("[TicTacToe] Winner status: {:?}", game.winner);

    if game.winner == GameWinner::Pending {
        game.next_turn();
    }

    ServerMessage::TicTacToe(TicTacToePayloadToClient {
        board: Some(serialize_board_as_numbers(game)),
        whos_turn: game.current_player_name().map(|s| s.to_string()),
        status: Some(format_status_with_names(game)),
    })
}

// --- Utility functions ---

/// ✅ FIXED: Serialize board as Vec<Vec<i32>> (number[][])
fn serialize_board_as_numbers(game: &TicTacToeModel) -> Vec<Vec<i32>> {
    game.board
        .iter()
        .map(|row| row.iter().map(|&v| v as i32).collect())
        .collect()
}

/// ✅ FIXED: Format status with actual player names and correct game over states
fn format_status_with_names(game: &TicTacToeModel) -> String {
    match game.winner {
        GameWinner::Pending => {
            // Return "IN_PROGRESS" during active game
            "IN_PROGRESS".to_string()
        }
        GameWinner::Player1 => "gameover_x".to_string(),
        GameWinner::Player2 => "gameover_o".to_string(),
        GameWinner::Tie => "gameover_draw".to_string(),
    }
}

fn parse_choice(choice: &str) -> Option<(usize, usize)> {
    match choice.to_uppercase().as_str() {
        "A1" => Some((0, 0)),
        "A2" => Some((0, 1)),
        "A3" => Some((0, 2)),
        "B1" => Some((1, 0)),
        "B2" => Some((1, 1)),
        "B3" => Some((1, 2)),
        "C1" => Some((2, 0)),
        "C2" => Some((2, 1)),
        "C3" => Some((2, 2)),
        _ => None,
    }
}