use std::sync::Arc;
use tokio::sync::mpsc::UnboundedSender;
use axum::extract::ws::Message;

use crate::types::{
    appstate::AppState,
    gameroom::GameRoom,
    types::{GameRoomPayload, ServerMessage},
};

/// Handles join/leave operations for game rooms.
pub async fn gameroom_handler(
    payload: GameRoomPayload,
    state: &Arc<AppState>,
    user_tx: UnboundedSender<Message>,
) -> ServerMessage {
    match payload.action.as_str() {
        "join" => handle_join(payload, state, user_tx).await,
        "leave" => handle_leave(payload, state, user_tx).await,
        _ => {
            let mut invalid = payload.clone();
            invalid.action = "invalid".into();
            ServerMessage::GameRoom(invalid)
        }
    }
}

/// A user joins a room
async fn handle_join(
    payload: GameRoomPayload,
    state: &Arc<AppState>,
    user_tx: UnboundedSender<Message>,
) -> ServerMessage {
    let mut rooms = state.rooms.write().await;

    let room = rooms.entry(payload.game_id.clone()).or_insert_with(GameRoom::default);

    // Add player if not already present
    if !room.users.contains(&payload.player_name) {
        room.users.push(payload.player_name.clone());
    }

    // Add the sender for broadcast
    room.txs.push(user_tx);

    ServerMessage::GameRoom(payload)
}

/// A user leaves a room
async fn handle_leave(
    payload: GameRoomPayload,
    state: &Arc<AppState>,
    user_tx: UnboundedSender<Message>,
) -> ServerMessage {
    let mut rooms = state.rooms.write().await;

    if let Some(room) = rooms.get_mut(&payload.game_id) {
        // Remove player
        room.users.retain(|u| u != &payload.player_name);

        // Remove sender (compare by channel identity)
        room.txs.retain(|tx| !tx.same_channel(&user_tx));

        // If empty, drop room
        if room.users.is_empty() {
            rooms.remove(&payload.game_id);
        }
    }

    ServerMessage::GameRoom(payload)
}
