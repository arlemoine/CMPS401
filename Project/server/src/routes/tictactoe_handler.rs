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
    // Which room is this socket in?
    let game_id = {
        let guard = current_room.read().await;
        match &*guard {
            Some(id) => id.clone(),
            None => {
                return ServerMessage::TicTacToe(TicTacToePayloadToClient {
                    board: vec!["".to_string(); 9],
                    whos_turn: "n/a".to_string(),
                    status: "no_room".to_string(),
                });
            }
        }
    };

    // Get a mutable handle to the rooms map, then the room
    let mut rooms = app_state.rooms.write().await;
    let room = match rooms.get_mut(&game_id) {
        Some(r) => r,
        None => {
            return ServerMessage::TicTacToe(TicTacToePayloadToClient {
                board: vec!["".to_string(); 9],
                whos_turn: "n/a".to_string(),
                status: "room_not_found".to_string(),
            });
        }
    };

    // Extract the TicTacToe model from the room
    let game = match &mut room.game {
        GameType::TicTacToe(m) => m,
        _ => {
            eprintln!("Tried to play TicTacToe in a non-TicTacToe room: {}", game_id);
            return ServerMessage::TicTacToe(TicTacToePayloadToClient {
                board: vec!["".to_string(); 9],
                whos_turn: "n/a".to_string(),
                status: "wrong_game_type".to_string(),
            });
        }
    };

    // Determine which Player this move is from
    let player = if room.users.get(0) == Some(&payload.whos_turn) {
        Player::Player1
    } else if room.users.get(1) == Some(&payload.whos_turn) {
        Player::Player2
    } else {
        return ServerMessage::TicTacToe(TicTacToePayloadToClient {
            board: serialize_board(game),
            whos_turn: format_player(game.whos_turn.clone()),
            status: "unknown_player".to_string(),
        });
    };

    // Check if it's actually this player's turn
    if player != game.whos_turn {
        return ServerMessage::TicTacToe(TicTacToePayloadToClient {
            board: serialize_board(game),
            whos_turn: format_player(game.whos_turn.clone()),
            status: "not_your_turn".to_string(),
        });
    }

    // Parse choice like "A1"
    let (row, col) = match parse_choice(&payload.choice) {
        Some(rc) => rc,
        None => {
            return ServerMessage::TicTacToe(TicTacToePayloadToClient {
                board: serialize_board(game),
                whos_turn: format_player(game.whos_turn.clone()),
                status: "invalid_choice".to_string(),
            });
        }
    };

    if !game.validate_choice(row, col) {
        return ServerMessage::TicTacToe(TicTacToePayloadToClient {
            board: serialize_board(game),
            whos_turn: format_player(game.whos_turn.clone()),
            status: "invalid_move".to_string(),
        });
    }

    game.mark_spot(row, col);
    game.check_winner();
    if game.winner == GameWinner::Pending {
        game.next_turn();
    }

    ServerMessage::TicTacToe(TicTacToePayloadToClient {
        board: serialize_board(game),
        whos_turn: format_player(game.whos_turn.clone()),
        status: format_status(game),
    })
}


// --- Utility functions ---
fn serialize_board(game: &TicTacToeModel) -> Vec<String> {
    game.board
        .iter()
        .flatten()
        .map(|&v| match v {
            1 => "X".to_string(),
            -1 => "O".to_string(),
            _ => "".to_string(),
        })
        .collect()
}

fn format_player(player: Player) -> String {
    match player {
        Player::Player1 => "x".to_string(),
        Player::Player2 => "o".to_string(),
    }
}

fn format_status(game: &TicTacToeModel) -> String {
    match game.winner {
        GameWinner::Pending => {
            if game.whos_turn == Player::Player1 {
                "next_x".to_string()
            } else {
                "next_o".to_string()
            }
        }
        GameWinner::Player1 => "gameover_x".to_string(),
        GameWinner::Player2 => "gameover_o".to_string(),
        GameWinner::Tie => "gameover_tie".to_string(),
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
