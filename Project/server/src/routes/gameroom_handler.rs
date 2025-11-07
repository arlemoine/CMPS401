use std::sync::Arc;
use tokio::sync::{
    mpsc::UnboundedSender,
    RwLock,
};
use axum::extract::ws::Message;

use crate::models::{
    appstate::AppState,
    gameroom::{GameRoom, GameType},
    tictactoe::model::TicTacToeModel,
};
use crate::types::{GameRoomPayload, ServerMessage};

/// Handles join/leave operations for game rooms.
pub async fn gameroom_handler(
    payload: GameRoomPayload,
    state: &Arc<AppState>,
    user_tx: UnboundedSender<Message>,
    current_room: Arc<RwLock<Option<String>>>,
) -> ServerMessage {
    match payload.action.as_str() {
        "join" => handle_join(payload, state, user_tx, current_room).await,
        "leave" => handle_leave(payload, state, user_tx).await,
        _ => {
            let mut invalid = payload.clone();
            invalid.action = "invalid".into();
            ServerMessage::GameRoom(invalid)
        }
    }
}

/// A user joins a room
pub async fn handle_join(
    payload: GameRoomPayload,
    state: &Arc<AppState>,
    user_tx: UnboundedSender<Message>,
    current_room: Arc<RwLock<Option<String>>>,
) -> ServerMessage {
    let mut rooms = state.rooms.write().await;

    // Match the requested game type and create the appropriate GameType
    let game_type = match payload.game.as_str() {
        "tictactoe" => GameType::TicTacToe(TicTacToeModel::new()),
        // Add other games here as needed:
        // "chess" => GameType::Chess(ChessModel::new()),
        // "checkers" => GameType::Checkers(CheckersModel::new()),
        other => {
            eprintln!("Unknown game type requested: {}", other);
            // Fallback: just return the payload without creating a room
            return ServerMessage::GameRoom(payload);
        }
    };

    // Insert new room if it doesn't exist
    let room = rooms.entry(payload.game_id.clone())
        .or_insert_with(|| GameRoom::new(
            payload.game_id.clone(),
            game_type 
        ));

    // Add player if not already present
    if !room.users.contains(&payload.player_name) {
        room.users.push(payload.player_name.clone());
    }
    let mut room_guard = current_room.write().await;
    *room_guard = Some(payload.game_id.clone());

    // Add the sender for broadcasting messages
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
