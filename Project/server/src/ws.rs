use axum::{
<<<<<<< HEAD
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};
use tokio::task::JoinHandle;
use tracing::{info, warn};
use futures_util::{SinkExt, StreamExt};
use futures_util::stream::SplitSink;
use tokio::sync::mpsc;

use crate::matchmaker::{broadcast_state, create_room, new_match_id, push_player, MatchRegistry};
use crate::types::{ClientMsg, MatchState, Player, ServerMsg};

pub async fn ws_upgrade(ws: WebSocketUpgrade, registry: MatchRegistry) -> impl IntoResponse {
    info!("/ws upgrade requested");
    ws.on_upgrade(|socket| handle_socket(socket, registry))
}

async fn handle_socket(socket: WebSocket, registry: MatchRegistry) {
    info!("websocket connected");

    let mut player_id: Option<String> = None;
    let mut display_name: Option<String> = None;

    let (ws_tx, mut ws_rx) = socket.split();
    let mut ws_tx_opt = Some(ws_tx);
    let (to_ws_tx, to_ws_rx) = mpsc::unbounded_channel::<Message>();
    let mut to_ws_rx_opt = Some(to_ws_rx);

    let mut bcast_forwarder: Option<JoinHandle<()>> = None;

    while let Some(Ok(msg)) = ws_rx.next().await {
        if let Message::Text(txt_bytes) = msg {
            let txt = txt_bytes.to_string();
            info!(%txt, "received text");

            match serde_json::from_str::<ClientMsg>(&txt) {
                Ok(ClientMsg::Join { display_name: name }) => {
                    if player_id.is_none() {
                        let id = new_match_id();
                        player_id = Some(id.clone());
                        display_name = Some(name.clone());
                        info!(%name, %id, "join message parsed - player ID assigned");
                    } else {
                        display_name = Some(name.clone());
                    }

                    let out = ServerMsg::Hello { server_version: "0.1" };
                    let s = serde_json::to_string(&out).unwrap();
                    let _ = to_ws_tx.send(Message::Text(s.into()));
                }

                Ok(ClientMsg::CreateMatch {}) => {
                    let match_id = new_match_id();
                    info!(%match_id, "create_match parsed; issuing match_id");

                    if let (Some(pid), Some(name)) = (&player_id, &display_name) {
                        let creator = Player {
                            id: pid.clone(),
                            display_name: name.clone(),
                            mark: "X".to_string(),
                        };
                        let state = MatchState {
                            match_id: match_id.clone(),
                            players: vec![creator.clone()],
                            status: "WAITING".to_string(),
                            board: vec![None; 9],
                            turn: None,
                        };

                        let (_id, rx) = create_room(&registry, state).await;

                        let out = ServerMsg::MatchCreated {
                            match_id: match_id.clone(),
                            you: creator,
                        };
                        let s = serde_json::to_string(&out).unwrap();
                        let _ = to_ws_tx.send(Message::Text(s.into()));

                        let _ = broadcast_state(&registry, &match_id).await;

                        if bcast_forwarder.is_none() {
                            if let (Some(ws_tx), Some(to_ws_rx)) = (ws_tx_opt.take(), to_ws_rx_opt.take())
                            {
                                bcast_forwarder = Some(spawn_broadcast_forwarder(rx, ws_tx, to_ws_rx));
                            }
                        }
                    } else {
                        let out = ServerMsg::Error {
                            code: "NO_PLAYER_ID".into(),
                            message: "Send join message first".into(),
                        };
                        let s = serde_json::to_string(&out).unwrap();
                        let _ = to_ws_tx.send(Message::Text(s.into()));
                    }
                }
            
            // In ws.rs, update the JoinMatch handler:
            
            Ok(ClientMsg::JoinMatch { match_id }) => {
                info!(%match_id, "join_match parsed");
            
                if player_id.is_none() || display_name.is_none() {
                    let out = ServerMsg::Error {
                        code: "NO_PLAYER_ID".into(),
                        message: "Send join message first".into(),
                    };
                    let s = serde_json::to_string(&out).unwrap();
                    let _ = to_ws_tx.send(Message::Text(s.into()));
                    continue;
                }
            
                let name = display_name.as_ref().unwrap().clone();
                let pid = player_id.as_ref().unwrap().clone();
            
                let mut reg = registry.write().await;
                if let Some(room) = reg.get_mut(&match_id) {
                    if room.state.players.len() >= 2 {
                        let out = ServerMsg::Error {
                            code: "MATCH_FULL".into(),
                            message: "Match already has 2 players".into(),
                        };
                        let s = serde_json::to_string(&out).unwrap();
                        let _ = to_ws_tx.send(Message::Text(s.into()));
                        continue;
                    }
            
                    // Add the second player
                    let you = push_player(room, pid.clone(), name.clone(), "O");
            
                    // Set match IN_PROGRESS if now 2 players
                    if room.state.players.len() == 2 {
                        room.state.status = "IN_PROGRESS".to_string();
                        room.state.turn = Some("X".to_string()); // X starts
                    }
            
                    // Send confirmation to joining player
                    let out = ServerMsg::JoinedMatch {
                        match_id: match_id.clone(),
                        you: you.clone(),
                    };
                    let s = serde_json::to_string(&out).unwrap();
                    let _ = to_ws_tx.send(Message::Text(s.into()));
            
                    // IMPORTANT: Subscribe the joining player to broadcast channel
                    let rx = room.bcast.subscribe();
                    
                    drop(reg); // Release the lock
            
                    // Set up broadcast forwarder for the joining player
                    if bcast_forwarder.is_none() {
                        if let (Some(ws_tx), Some(to_ws_rx)) = (ws_tx_opt.take(), to_ws_rx_opt.take()) {
                            bcast_forwarder = Some(spawn_broadcast_forwarder(rx, ws_tx, to_ws_rx));
                        }
                    }
            
                    // Broadcast updated state to all players (including the new one)
                    let _ = broadcast_state(&registry, &match_id).await;
            
                    info!(%match_id, player_id=%pid, "player joined; state broadcasted");
                } else {
                    drop(reg);
                    let out = ServerMsg::Error {
                        code: "MATCH_NOT_FOUND".into(),
                        message: format!("Match {} not found", match_id),
                    };
                    let s = serde_json::to_string(&out).unwrap();
                    let _ = to_ws_tx.send(Message::Text(s.into()));
                }
            }

                
                // In ws.rs, update the MakeMove handler section:
                
                Ok(ClientMsg::MakeMove { match_id, index }) => {
                    info!(%match_id, index = index, "make_move parsed");
                
                    if index >= 9 {
                        let out = ServerMsg::Error {
                            code: "BAD_INDEX".into(),
                            message: "Index must be 0..8".into(),
                        };
                        let s = serde_json::to_string(&out).unwrap();
                        let _ = to_ws_tx.send(Message::Text(s.into()));
                        continue;
                    }
                
                    let mut reg = registry.write().await;
                    if let Some(room) = reg.get_mut(&match_id) {
                        let pid = match &player_id {
                            Some(p) => p.clone(),
                            None => {
                                let out = ServerMsg::Error {
                                    code: "NO_PLAYER".into(),
                                    message: "Player not identified".into(),
                                };
                                let s = serde_json::to_string(&out).unwrap();
                                let _ = to_ws_tx.send(Message::Text(s.into()));
                                continue;
                            }
                        };
                        
                        // Find the player making the move
                        let player_mark = match room.state.players.iter().find(|p| p.id == pid) {
                            Some(player) => player.mark.clone(),
                            None => {
                                let out = ServerMsg::Error {
                                    code: "NOT_IN_MATCH".into(),
                                    message: "You are not in this match".into(),
                                };
                                let s = serde_json::to_string(&out).unwrap();
                                let _ = to_ws_tx.send(Message::Text(s.into()));
                                continue;
                            }
                        };
                
                        // Validate game state and turn
                        if room.state.status != "IN_PROGRESS" { 
                            let out = ServerMsg::Error {
                                code: "GAME_NOT_STARTED".into(),
                                message: "Game is not in progress".into(),
                            };
                            let s = serde_json::to_string(&out).unwrap();
                            let _ = to_ws_tx.send(Message::Text(s.into()));
                            continue;
                        }
                        
                        if room.state.turn.as_deref() != Some(player_mark.as_str()) { 
                            let out = ServerMsg::Error {
                                code: "NOT_YOUR_TURN".into(),
                                message: "It's not your turn".into(),
                            };
                            let s = serde_json::to_string(&out).unwrap();
                            let _ = to_ws_tx.send(Message::Text(s.into()));
                            continue;
                        }
                        
                        if room.state.board[index].is_some() { 
                            let out = ServerMsg::Error {
                                code: "CELL_OCCUPIED".into(),
                                message: "This cell is already occupied".into(),
                            };
                            let s = serde_json::to_string(&out).unwrap();
                            let _ = to_ws_tx.send(Message::Text(s.into()));
                            continue;
                        }
                
                        // Make the move
                        room.state.board[index] = Some(player_mark.clone());
                
                        // Check for winner or draw
                        if let Some(winner_mark) = check_winner(&room.state.board) {
                            room.state.status = "FINISHED".to_string();
                            room.state.turn = None;
                            info!(%match_id, winner=%winner_mark, "Game finished with winner");
                        } else if room.state.board.iter().all(|c| c.is_some()) {
                            room.state.status = "FINISHED".to_string(); // draw
                            room.state.turn = None;
                            info!(%match_id, "Game finished as draw");
                        } else {
                            // Switch turn
                            room.state.turn = Some(if player_mark == "X" { 
                                "O".to_string() 
                            } else { 
                                "X".to_string() 
                            });
                            info!(%match_id, next_turn=?room.state.turn, "Turn switched");
                        }
                
                        drop(reg); // Release the lock before broadcasting
                        
                        // Broadcast updated state to all players
                        let _ = broadcast_state(&registry, &match_id).await;
                        info!(%match_id, "State broadcasted after move");
                    } else {
                        let out = ServerMsg::Error {
                            code: "MATCH_NOT_FOUND".into(),
                            message: format!("Match {} not found", match_id),
                        };
                        let s = serde_json::to_string(&out).unwrap();
                        let _ = to_ws_tx.send(Message::Text(s.into()));
                    }
                }

                Err(e) => {
                    warn!(error = %e, "bad json");
                    let out = ServerMsg::Error {
                        code: "BAD_JSON".into(),
                        message: e.to_string(),
                    };
                    let s = serde_json::to_string(&out).unwrap();
                    let _ = to_ws_tx.send(Message::Text(s.into()));
=======
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt};
use std::sync::Arc;

use crate::types::{
    appstate::AppState,
    types::{
        ClientMessage,
        EchoPayload,
        ServerMessage,
    }
};
use crate::routes::{
    echo::echo_handler,
    join_game::join_game,
};

#[axum::debug_handler]
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

/// Helper to parse JSON string -> ClientMessage
fn parse_client_message(text: &str) -> Result<ClientMessage, String> {
    serde_json::from_str::<ClientMessage>(text)
        .map_err(|_| "Invalid JSON format for ClientMessage".into())
}

pub async fn handle_socket(socket: WebSocket, state: Arc<AppState>) {
    // Split the socket into a transmitter and receiver
    let (mut tx, mut rx) = socket.split();

    while let Some(Ok(msg)) = rx.next().await {
        if let Message::Text(text) = msg {
            match parse_client_message(&text) {
                Ok(client_msg) => {
                    handle_client_message(client_msg, &state, &mut tx).await;
                }
                Err(err_str) => {
                    let _ = send_server_message(
                        ServerMessage::Echo(EchoPayload { message: err_str }),
                        &mut tx,
                    )
                    .await;
>>>>>>> origin/dev
                }
            }
        }
    }
<<<<<<< HEAD

    if let Some(task) = bcast_forwarder {
        task.abort();
    }
    info!("websocket closed");
}

/// Forwards room broadcasts and app-initiated messages → this socket.
fn spawn_broadcast_forwarder(
    mut rx: tokio::sync::broadcast::Receiver<ServerMsg>,
    mut ws_tx: SplitSink<WebSocket, Message>,
    mut to_ws_rx: mpsc::UnboundedReceiver<Message>,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        loop {
            tokio::select! {
                Some(msg) = to_ws_rx.recv() => {
                    if ws_tx.send(msg).await.is_err() { break; }
                }
                res = rx.recv() => {
                    match res {
                        Ok(server_msg) => {
                            if let Ok(text) = serde_json::to_string(&server_msg) {
                                if ws_tx.send(Message::Text(text.into())).await.is_err() { break; }
                            }
                        }
                        Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
                        Err(_) => break,
                    }
                }
            }
        }
    })
}

/// Simple tic-tac-toe winner detection
pub fn check_winner(board: &Vec<Option<String>>) -> Option<String> {
    let wins = [
        (0,1,2),(3,4,5),(6,7,8),
        (0,3,6),(1,4,7),(2,5,8),
        (0,4,8),(2,4,6),
    ];
    for (a,b,c) in wins.iter() {
        if let (Some(x), Some(y), Some(z)) = (&board[*a], &board[*b], &board[*c]) {
            if x == y && y == z {
                return Some(x.clone());
            }
        }
    }
    None
=======
}

/// Central switchboard for different ClientMessage types
async fn handle_client_message(
    msg: ClientMessage,
    state: &Arc<AppState>,
    tx: &mut futures::stream::SplitSink<WebSocket, Message>,
) {
    match msg {
        ClientMessage::Echo(payload) => {
            let response = echo_handler(payload, &state);
            if let Ok(json_str) = serde_json::to_string(&response) {
                let _ = tx.send(Message::Text(json_str.into())).await;
            }
        }
        ClientMessage::GameRoom(payload) => {
            // Call the join/create game logic, get a response message
            let response_msg = join_game(payload, state).await;
            send_server_message(response_msg, tx).await;
        }
        // You can add MovePiece, Chat, etc. here in the future
    }
}

/// Helper to serialize ServerMessage -> send over socket
async fn send_server_message(msg: ServerMessage, tx: &mut futures::stream::SplitSink<WebSocket, Message>) {
    if let Ok(json_str) = serde_json::to_string(&msg) {
        let _ = tx.send(Message::Text(json_str.into())).await;
    }
>>>>>>> origin/dev
}
