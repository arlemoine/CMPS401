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
use crate::types::{GameRoomPayload, ServerMessage, TicTacToePayloadToClient};

/// Handles join/leave operations for game rooms.
pub async fn gameroom_handler(
    payload: GameRoomPayload,
    state: &Arc<AppState>,
    user_tx: UnboundedSender<Message>,
    current_room: Arc<RwLock<Option<String>>>,
) -> ServerMessage {
    match payload.action.as_str() {
        "join" => handle_join(payload, state, user_tx, current_room).await,
        "leave" => handle_leave(payload, state, user_tx, current_room).await,
        "reset" => handle_reset(payload, state).await,
        _ => {
            let mut invalid = payload.clone();
            invalid.action = "invalid".into();
            ServerMessage::GameRoom(invalid)
        }
    }
}

/// A user joins a room
pub async fn handle_join(
    mut payload: GameRoomPayload,
    state: &Arc<AppState>,
    user_tx: UnboundedSender<Message>,
    current_room: Arc<RwLock<Option<String>>>,
) -> ServerMessage {
    let mut rooms = state.rooms.write().await;

    // Match the requested game type and create the appropriate GameType
    let game_type = match payload.game.as_str() {
        "tictactoe" => GameType::TicTacToe(TicTacToeModel::new()),
        other => {
            eprintln!("Unknown game type requested: {}", other);
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
        println!("[GameRoom] {} joined room {}. Players: {:?}",
                 payload.player_name, payload.game_id, room.users);
    }

    // Add the sender if not already present
    if !room.txs.iter().any(|tx| tx.same_channel(&user_tx)) {
        room.txs.push(user_tx);
    }

    let mut room_guard = current_room.write().await;
    *room_guard = Some(payload.game_id.clone());

    // Clone the complete player list
    let all_players = room.users.clone();
    
    // ✅ If we now have 2 players, initialize the game and send initial state
    let should_start_game = all_players.len() == 2;
    let initial_game_state = if should_start_game {
        if let GameType::TicTacToe(game) = &mut room.game {
            // Assign player names
            game.player1_name = Some(all_players[0].clone());
            game.player2_name = Some(all_players[1].clone());
            
            println!("[GameRoom] Game starting! Player 1 (X): {}, Player 2 (O): {}",
                     all_players[0], all_players[1]);
            
            // Create initial game state message
            Some(TicTacToePayloadToClient {
                board: Some(vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]),
                whos_turn: Some(all_players[0].clone()), // Player 1 starts
                status: Some("IN_PROGRESS".to_string()),
            })
        } else {
            None
        }
    } else {
        None
    };
    
    drop(room_guard);
    drop(rooms);

    // Add players list to response
    payload.players = Some(all_players);
    
    // ✅ Broadcast initial game state if game just started
    if let Some(game_state) = initial_game_state {
        let rooms = state.rooms.read().await;
        if let Some(room) = rooms.get(&payload.game_id) {
            let game_msg = ServerMessage::TicTacToe(game_state);
            let serialized = serde_json::to_string(&game_msg).unwrap();
            for tx in &room.txs {
                let _ = tx.send(Message::Text(serialized.clone().into()));
            }
        }
    }

    ServerMessage::GameRoom(payload)
}

/// A user leaves a room
async fn handle_leave(
    mut payload: GameRoomPayload,
    state: &Arc<AppState>,
    user_tx: UnboundedSender<Message>,
    current_room: Arc<RwLock<Option<String>>>,
) -> ServerMessage {
    let mut rooms = state.rooms.write().await;

    if let Some(room) = rooms.get_mut(&payload.game_id) {
        // Remove player from users list
        room.users.retain(|u| u != &payload.player_name);
        println!("[GameRoom] {} left room {}. Remaining players: {:?}",
                 payload.player_name, payload.game_id, room.users);

        // Remove sender
        room.txs.retain(|tx| !tx.same_channel(&user_tx));
        
        // ✅ Update payload with remaining players
        payload.players = Some(room.users.clone());

        // If empty, drop room
        if room.users.is_empty() {
            rooms.remove(&payload.game_id);
            println!("[GameRoom] Room {} removed (empty)", payload.game_id);
        }
    }

    // Clear current_room tracker
    *current_room.write().await = None;

    ServerMessage::GameRoom(payload)
}

/// Reset game state - allows players to play again
async fn handle_reset(
    mut payload: GameRoomPayload,
    state: &Arc<AppState>,
) -> ServerMessage {
    let mut rooms = state.rooms.write().await;

    if let Some(room) = rooms.get_mut(&payload.game_id) {
        println!("[GameRoom] Resetting game in room {}", payload.game_id);
        
        // Reset the game model and broadcast new game state
        match &mut room.game {
            GameType::TicTacToe(model) => {
                // Preserve player names
                let p1 = model.player1_name.clone();
                let p2 = model.player2_name.clone();
                
                // Reset the model
                *model = TicTacToeModel::new();
                model.player1_name = p1.clone();
                model.player2_name = p2.clone();
                
                println!("[GameRoom] TicTacToe game reset. Players: {:?} vs {:?}", p1, p2);
                
                // ✅ Broadcast fresh game state to all players in the room
                if let (Some(player1), Some(player2)) = (p1, p2) {
                    let game_state = TicTacToePayloadToClient {
                        board: Some(vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]),
                        whos_turn: Some(player1.clone()), // Player 1 starts
                        status: Some("IN_PROGRESS".to_string()),
                    };
                    
                    let game_msg = ServerMessage::TicTacToe(game_state);
                    let serialized = serde_json::to_string(&game_msg).unwrap();
                    
                    for tx in &room.txs {
                        let _ = tx.send(Message::Text(serialized.clone().into()));
                    }
                }
            }
        }
        
        // ✅ Include player list in response
        payload.players = Some(room.users.clone());
    }

    ServerMessage::GameRoom(payload)
}