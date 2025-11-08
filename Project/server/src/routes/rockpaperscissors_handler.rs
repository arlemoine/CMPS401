use std::sync::Arc;
use tokio::sync::RwLock;

use crate::models::{
    appstate::AppState,
    gameroom::GameType,
    rockpaperscissors::model::{RockPaperScissorsModel, RpsChoice, RpsRoundResult},
};
use crate::types::{
    RockPaperScissorsPayloadToClient,
    RockPaperScissorsPayloadToServer,
    ServerMessage,
};

/// Handles RockPaperScissors messages sent by clients.
pub async fn rockpaperscissors_handler(
    payload: RockPaperScissorsPayloadToServer,
    app_state: &Arc<AppState>,
    _current_room: Arc<RwLock<Option<String>>>,
) -> ServerMessage {
    let game_id = payload.game_id.clone();
    let mut rooms = app_state.rooms.write().await;

    let Some(room) = rooms.get_mut(&game_id) else {
        return build_error_payload(game_id, None, "room_not_found", "Room not found.");
    };

    let game = match &mut room.game {
        GameType::RockPaperScissors(model) => model,
        _ => {
            return build_error_payload(
                game_id,
                None,
                "wrong_game_type",
                "This room is not a RockPaperScissors game.",
            );
        }
    };

    // Assign player names from the room roster if they haven't been set yet.
    if game.player1_name.is_none() && !room.users.is_empty() {
        game.player1_name = Some(room.users[0].clone());
    }
    if game.player2_name.is_none() && room.users.len() > 1 {
        game.player2_name = Some(room.users[1].clone());
    }

    if !game.both_players_joined() {
        return ServerMessage::RockPaperScissors(RockPaperScissorsPayloadToClient {
            game_id,
            player1: game.player1_name.clone(),
            player2: game.player2_name.clone(),
            player1_choice: None,
            player2_choice: None,
            status: "waiting_for_opponent".to_string(),
            winner: None,
            message: Some("Waiting for another player to join.".to_string()),
        });
    }

    if let Some(choice_str) = payload.choice.as_deref() {
        let Some(choice) = RpsChoice::from_str(choice_str) else {
            return build_error_payload(
                game_id,
                Some(&*game),
                "invalid_choice",
                "Choice must be rock, paper, or scissors.",
            );
        };

        if let Err(_) = game.submit_choice(&payload.player_name, choice) {
            return build_error_payload(
                game_id,
                Some(&*game),
                "unknown_player",
                "Only players in this room may submit choices.",
            );
        }
    }

    if game.both_choices_made() {
        game.resolve_round();
    }

    ServerMessage::RockPaperScissors(build_state_payload(&game_id, game))
}

fn build_state_payload(game_id: &str, game: &RockPaperScissorsModel) -> RockPaperScissorsPayloadToClient {
    let reveal_choices = game.both_choices_made() && game.winner != RpsRoundResult::Pending;

    let (status, message) = if !game.both_players_joined() {
        (
            "waiting_for_opponent".to_string(),
            Some("Waiting for another player to join.".to_string()),
        )
    } else if reveal_choices {
        let msg = match game.winner {
            RpsRoundResult::Tie => "Round ended in a tie.".to_string(),
            RpsRoundResult::Player1 | RpsRoundResult::Player2 => match game.winner_name() {
                Some(name) => format!("{} wins this round!", name),
                None => "Round complete.".to_string(),
            },
            RpsRoundResult::Pending => "Round in progress.".to_string(),
        };
        ("round_complete".to_string(), Some(msg))
    } else if game.player1_choice.is_some() || game.player2_choice.is_some() {
        (
            "waiting_for_opponent_choice".to_string(),
            Some("Waiting for the other player to lock in.".to_string()),
        )
    } else {
        (
            "waiting_for_choices".to_string(),
            Some("Choose rock, paper, or scissors.".to_string()),
        )
    };

    RockPaperScissorsPayloadToClient {
        game_id: game_id.to_string(),
        player1: game.player1_name.clone(),
        player2: game.player2_name.clone(),
        player1_choice: if reveal_choices {
            game.player1_choice.map(|c| c.as_str().to_string())
        } else {
            None
        },
        player2_choice: if reveal_choices {
            game.player2_choice.map(|c| c.as_str().to_string())
        } else {
            None
        },
        status,
        winner: match game.winner {
            RpsRoundResult::Player1 => game.player1_name.clone(),
            RpsRoundResult::Player2 => game.player2_name.clone(),
            RpsRoundResult::Tie => Some("tie".to_string()),
            RpsRoundResult::Pending => None,
        },
        message,
    }
}

fn build_error_payload(
    game_id: String,
    game: Option<&RockPaperScissorsModel>,
    status: &str,
    message: &str,
) -> ServerMessage {
    let (player1, player2) = if let Some(model) = game {
        (model.player1_name.clone(), model.player2_name.clone())
    } else {
        (None, None)
    };

    ServerMessage::RockPaperScissors(RockPaperScissorsPayloadToClient {
        game_id,
        player1,
        player2,
        player1_choice: None,
        player2_choice: None,
        status: status.to_string(),
        winner: None,
        message: Some(message.to_string()),
    })
}
