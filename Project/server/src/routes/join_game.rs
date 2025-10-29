use std::sync::Arc;
use crate::types::{
    appstate::AppState,
    gameroom::GameRoom,
    types::{
        GameRoomPayload,
        ServerMessage,
    }
};

pub async fn join_game(payload: GameRoomPayload, state: &Arc<AppState>) -> ServerMessage {
    let mut rooms = state.rooms.write().await;

    if let Some(room) = rooms.get_mut(&payload.game_id) {
        let mut users = room.users.write().await;
        if !users.contains(&payload.player_name) {
            users.push(payload.player_name.clone());
        }
        drop(users);
        ServerMessage::GameRoom(payload)
    } else {
        // Room doesn't exist -> create it
        let room = GameRoom::default();
        {
            let mut users = room.users.write().await;
            users.push(payload.player_name.clone());
        }
        rooms.insert(payload.game_id.clone(), room);
        ServerMessage::GameRoom(payload)
    }
}
