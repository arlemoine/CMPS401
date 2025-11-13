// routes/airhockey_handler.rs

use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::Utc;
use crate::types::{ServerMessage, AirHockeyPayloadToClient, AirHockeyPayloadToServer};
use crate::models::gameroom::{GameRoom, GameType};
use crate::models::airhockey::model::{AirHockeyModel, PlayerSlot};
use crate::models::appstate::AppState;

/// Handles incoming Air Hockey messages from clients
pub async fn airhockey_handler(
    payload: AirHockeyPayloadToServer,
    app_state: &Arc<AppState>,
    _current_room: Arc<RwLock<Option<String>>>,
) -> ServerMessage {
    let game_id = payload.game_id.clone();

    // Get mutable room
    let mut rooms = app_state.rooms.write().await;
    let room = match rooms.get_mut(&game_id) {
        Some(r) => r,
        None => {
            eprintln!("[AirHockey] Room not found: {}", game_id);
            return ServerMessage::AirHockey(AirHockeyPayloadToClient {
                event: "room_not_found".to_string(),
                game_id,
                timestamp: 0.0,
                paddles: Default::default(),
                puck: Default::default(),
                score: Default::default(),
            });
        }
    };

    // Extract AirHockey model
    let game = match &mut room.game {
        GameType::AirHockey(m) => m,
        _ => {
            eprintln!("[AirHockey] Wrong game type in room: {}", game_id);
            return ServerMessage::AirHockey(AirHockeyPayloadToClient {
                event: "wrong_game_type".to_string(),
                game_id,
                timestamp: 0.0,
                paddles: Default::default(),
                puck: Default::default(),
                score: Default::default(),
            });
        }
    };

    // Assign players if not already assigned
    if !game.player_slots.contains_key(&PlayerSlot::Player1) && !room.users.is_empty() {
        let p1 = room.users[0].clone();
        game.assign_player(p1);
    }
    if !game.player_slots.contains_key(&PlayerSlot::Player2) && room.users.len() > 1 {
        let p2 = room.users[1].clone();
        game.assign_player(p2);
    }

    // Handle payload actions
    if payload.action == "move_paddle" {
        if let (Some(pos), Some(vel)) = (payload.position, payload.velocity) {
            // Map player_id to player_number (1 or 2)
            if let Some(player_number) = game.player_slots.iter()
                .find_map(|(slot, id)| {
                    if id == &payload.player_id {
                        Some(match slot {
                            PlayerSlot::Player1 => 1,
                            PlayerSlot::Player2 => 2,
                        })
                    } else { None }
                }) 
            {
                game.update_paddle(player_number, Some(pos), Some(vel));
            }
        }
    }

    // Tick the game
    let dt = 1.0 / 60.0;
    game.tick(dt as f32);

    // Build server payload
    ServerMessage::AirHockey(AirHockeyPayloadToClient {
        event: "update".to_string(),
        game_id: game_id.clone(),
        timestamp: Utc::now().timestamp_millis() as f64 / 1000.0,
        paddles: game.table.paddles.clone(),
        puck: game.table.puck.clone(),
        score: game.table.score.clone(),
    })
}
