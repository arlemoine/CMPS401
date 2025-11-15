use std::sync::Arc;
use tokio::sync::{
    mpsc::UnboundedSender,
    RwLock,
};
use axum::extract::ws::Message;
use crate::models::{
    appstate::AppState,
    gameroom::{GameRoom, GameType},
    rockpaperscissors::model::RockPaperScissorsModel,
    tictactoe::model::TicTacToeModel,
    uno::model::UnoModel,
    uno::model::UnoColor,
};
use crate::types::{
    GameRoomPayload,
    RockPaperScissorsPayloadToClient,
    ServerMessage,
    TicTacToePayloadToClient,
    UnoPayloadToClient,
};

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
        "rockpaperscissors" => GameType::RockPaperScissors(RockPaperScissorsModel::new()),
        "uno" => GameType::Uno(UnoModel::new()),
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
    
    // ✅ Build a lobby/initial snapshot to broadcast
    // - For TicTacToe and RPS: only when exactly 2 players
    // - For Uno: on every join (so FE always sees up-to-date lobby state and can start when players.len() >= 2)
    let initial_game_state: Option<ServerMessage> = match &mut room.game {
        GameType::TicTacToe(game) => {
            if all_players.len() == 2 {
                game.player1_name = Some(all_players[0].clone());
                game.player2_name = Some(all_players[1].clone());

                println!(
                    "[GameRoom] TicTacToe starting! Player 1 (X): {}, Player 2 (O): {}",
                    all_players[0], all_players[1]
                );

                Some(ServerMessage::TicTacToe(TicTacToePayloadToClient {
                    board: Some(vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]),
                    whos_turn: Some(all_players[0].clone()), // Player 1 starts
                    status: Some("IN_PROGRESS".to_string()),
                }))
            } else { None }
        }
        GameType::RockPaperScissors(game) => {
            if all_players.len() == 2 {
                game.player1_name = Some(all_players[0].clone());
                game.player2_name = Some(all_players[1].clone());
                game.reset_round();

                println!(
                    "[GameRoom] RockPaperScissors starting! Player 1: {}, Player 2: {}",
                    all_players[0], all_players[1]
                );

                Some(ServerMessage::RockPaperScissors(
                    RockPaperScissorsPayloadToClient {
                        game_id: payload.game_id.clone(),
                        player1: Some(all_players[0].clone()),
                        player2: Some(all_players[1].clone()),
                        player1_choice: None,
                        player2_choice: None,
                        status: "waiting_for_choices".to_string(),
                        winner: None,
                        message: Some("Both players joined. Make your selection!".to_string()),
                    },
                ))
            } else { None }
        }
        GameType::Uno(game) => {
            // Register current players in the Uno model
            for player in &all_players {
                if !game.players.contains(player) {
                    game.add_player(player);
                }
            }

            if game.started {
                // If a round is already in progress, emit the current game snapshot
                Some(ServerMessage::Uno(UnoPayloadToClient {
                    game_id: payload.game_id.clone(),
                    players: Some(all_players.clone()),
                    current_idx: Some(game.current_idx as i32),
                    direction: Some(game.direction),
                    top_discard: game.discard_top.clone(),
                    chosen_color: game.chosen_color.as_ref().map(|c| match c {
                        UnoColor::Red => "Red".to_string(),
                        UnoColor::Yellow => "Yellow".to_string(),
                        UnoColor::Green => "Green".to_string(),
                        UnoColor::Blue => "Blue".to_string(),
                        UnoColor::Wild => "Wild".to_string(),
                    }),
                    pending_draw: Some(game.pending_draw),
                    public_counts: Some(game.public_counts()),
                    hand: None,
                    winner: game.winner.clone(),
                }))
            } else {
                // Otherwise, emit a lobby snapshot so FE can enable Start when players.len() >= 2
                let counts = vec![0u8; all_players.len()];
                Some(ServerMessage::Uno(UnoPayloadToClient {
                    game_id: payload.game_id.clone(),
                    players: Some(all_players.clone()),
                    current_idx: Some(0),
                    direction: Some(1),
                    top_discard: None,
                    chosen_color: None,
                    pending_draw: Some(0),
                    public_counts: Some(counts),
                    hand: None,
                    winner: None,
                }))
            }
        }
    };
    
    drop(room_guard);
    drop(rooms);

    // Add players list to response
    payload.players = Some(all_players);
    
    // ✅ Broadcast initial game state if game just started
    if let Some(game_state) = initial_game_state {
        let rooms = state.rooms.read().await;
        if let Some(room) = rooms.get(&payload.game_id) {
            let serialized = serde_json::to_string(&game_state).unwrap();
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
            GameType::RockPaperScissors(model) => {
                let p1 = model.player1_name.clone();
                let p2 = model.player2_name.clone();
                model.reset_round();

                println!(
                    "[GameRoom] RockPaperScissors game reset. Players: {:?} vs {:?}",
                    p1, p2
                );

                if let (Some(player1), Some(player2)) = (p1, p2) {
                    let payload = RockPaperScissorsPayloadToClient {
                        game_id: payload.game_id.clone(),
                        player1: Some(player1.clone()),
                        player2: Some(player2.clone()),
                        player1_choice: None,
                        player2_choice: None,
                        status: "waiting_for_choices".to_string(),
                        winner: None,
                        message: Some("Round reset. Choose rock, paper, or scissors.".to_string()),
                    };

                    let game_msg = ServerMessage::RockPaperScissors(payload);
                    let serialized = serde_json::to_string(&game_msg).unwrap();

                    for tx in &room.txs {
                        let _ = tx.send(Message::Text(serialized.clone().into()));
                    }
                }
            }
            
            GameType::Uno(model) => {
                // Reset UNO model to lobby state and preserve current room players
                *model = UnoModel::new();
                let players = room.users.clone();
                for p in &players { model.add_player(p); }

                println!(
                    "[GameRoom] UNO game reset. Players: {:?}",
                    players
                );

                // Broadcast a fresh lobby snapshot so FE can start when players.len() >= 2
                let counts = vec![0u8; players.len()];
                let payload_uno = UnoPayloadToClient {
                    game_id: payload.game_id.clone(),
                    players: Some(players.clone()),
                    current_idx: Some(0),
                    direction: Some(1),
                    top_discard: None,
                    chosen_color: None,
                    pending_draw: Some(0),
                    public_counts: Some(counts),
                    hand: None,
                    winner: None,
                };

                let game_msg = ServerMessage::Uno(payload_uno);
                let serialized = serde_json::to_string(&game_msg).unwrap();

                for tx in &room.txs {
                    let _ = tx.send(Message::Text(serialized.clone().into()));
                }
            }
        }
        
        // ✅ Include player list in response
        payload.players = Some(room.users.clone());
    }

    ServerMessage::GameRoom(payload)
}
