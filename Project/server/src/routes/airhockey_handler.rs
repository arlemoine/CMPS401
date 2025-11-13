use std::sync::Arc;
use chrono::Utc;

use crate::{
    models::{
        gameroom::{GameType},
    },
    types::{
        AirHockeyPayloadToServer,
        AirHockeyPayloadToClient,
        PaddleState,
        PuckState,
        ServerMessage,
    },
    models::appstate::AppState,
};

use tokio::sync::RwLock;

pub async fn airhockey_handler(
    payload: AirHockeyPayloadToServer,
    state: &Arc<AppState>,
    _current_room: Arc<RwLock<Option<String>>>,
) -> ServerMessage {
    let mut rooms = state.rooms.write().await;

    // Get the corresponding room
    let Some(room) = rooms.get_mut(&payload.game_id) else {
        return ServerMessage::Echo(crate::types::EchoPayload {
            message: format!("No AirHockey room found for {}", payload.game_id),
        });
    };

    // Make sure it’s actually an AirHockey room
    let GameType::AirHockey(model) = &mut room.game else {
        return ServerMessage::Echo(crate::types::EchoPayload {
            message: format!("Room {} is not an AirHockey game", payload.game_id),
        });
    };

    // Determine player number (1 or 2)
    let player_number = model
        .players
        .iter()
        .find_map(|(num, id)| if *id == payload.player_id { Some(*num) } else { None });

    match payload.action.as_str() {
        "move_paddle" => {
            if let Some(player_number) = player_number {
                model.update_paddle(
                    player_number,
                    payload.position.clone(),
                    payload.velocity.clone(),
                );
            }

            // Update physics
            model.tick(0.016);
        }

        "request_state" => {
            // Do nothing; just return state below
        }

        other => {
            return ServerMessage::Echo(crate::types::EchoPayload {
                message: format!("Unknown AirHockey action: {}", other),
            });
        }
    }

    // Prepare updated state for all clients
    let paddles = model
        .table
        .paddles
        .iter()
        .map(|(num, paddle)| {
            let player_id = model.players.get(num).unwrap().clone();
            (
                player_id,
                PaddleState {
                    x: paddle.position.x,
                    y: paddle.position.y,
                    vx: paddle.velocity.x,
                    vy: paddle.velocity.y,
                },
            )
        })
        .collect();

    let puck = &model.table.puck;
    let score = model
        .table
        .score
        .iter()
        .map(|(num, val)| {
            let player_id = model.players.get(num).unwrap().clone();
            (player_id, *val)
        })
        .collect();

    ServerMessage::AirHockey(AirHockeyPayloadToClient {
        event: "update".into(),
        game_id: payload.game_id.clone(),
        timestamp: Utc::now().timestamp_millis() as f64,
        paddles,
        puck: PuckState {
            x: puck.position.x,
            y: puck.position.y,
            vx: puck.velocity.x,
            vy: puck.velocity.y,
        },
        score,
    })
}
