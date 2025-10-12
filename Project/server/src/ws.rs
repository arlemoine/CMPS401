use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};
use tokio::task::JoinHandle;
use tracing::{info, warn};
use futures_util::{SinkExt, StreamExt};
use futures_util::stream::SplitSink;
use tokio::sync::mpsc;

use crate::matchmaker::{
    broadcast_state, create_room, new_match_id, push_player, MatchRegistry,
};
use crate::types::{ClientMsg, MatchState, Player, ServerMsg};

pub async fn ws_upgrade(ws: WebSocketUpgrade, registry: MatchRegistry) -> impl IntoResponse {
    info!("/ws upgrade requested");
    ws.on_upgrade(|socket| handle_socket(socket, registry))
}

async fn handle_socket(socket: WebSocket, registry: MatchRegistry) {
    info!("websocket connected");
    let mut player_id: Option<String> = None;
    let mut display_name: Option<String> = None;

    // Split the socket into sender/receiver and create a channel for app-initiated sends
    let (ws_tx, mut ws_rx) = socket.split();
    let mut ws_tx_opt = Some(ws_tx);
    let (to_ws_tx, to_ws_rx) = mpsc::unbounded_channel::<Message>();
    let mut to_ws_rx_opt = Some(to_ws_rx);

    // Each connection may have a background task that forwards room broadcasts → this socket.
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
                        info!(%name, "join message parsed - updating display name");
                        display_name = Some(name.clone());
                    }
                    let out = ServerMsg::Hello {
                        server_version: "0.1",
                    };
                    let s = serde_json::to_string(&out).unwrap();
                    if to_ws_tx.send(Message::Text(s.into())).is_err() {
                        break;
                    }
                }

                Ok(ClientMsg::CreateMatch {}) => {
                    let match_id = new_match_id();
                    info!(%match_id, "create_match parsed; issuing match_id");

                    if let (Some(pid), Some(name)) = (&player_id, &display_name) {
                        // Creator is X
                        let creator = Player {
                            id: pid.clone(),
                            display_name: name.clone(),
                            mark: "X".to_string(),
                        };
                        let state = MatchState {
                            match_id: match_id.clone(),
                            players: vec![creator.clone()],
                            status: "WAITING".to_string(),
                        };

                        // Create the room and subscribe this socket to its broadcast.
                        let (_id, rx) = create_room(&registry, state).await;

                        // Ack creator
                        let out = ServerMsg::MatchCreated {
                            match_id: match_id.clone(),
                            you: creator,
                        };
                        let s = serde_json::to_string(&out).unwrap();
                        if to_ws_tx.send(Message::Text(s.into())).is_err() {
                            break;
                        }

                        // Send initial snapshot via broadcast path
                        let _ = broadcast_state(&registry, &match_id).await;

                        // Start forwarding broadcasts → this socket if not already started
                        if bcast_forwarder.is_none() {
                            if let (Some(ws_tx), Some(to_ws_rx)) = (ws_tx_opt.take(), to_ws_rx_opt.take()) {
                                bcast_forwarder = Some(spawn_broadcast_forwarder(rx, ws_tx, to_ws_rx));
                            }
                        }
                    } else {
                        let out = ServerMsg::Error {
                            code: "NO_PLAYER_ID".into(),
                            message: "Send join message first".into(),
                        };
                        let s = serde_json::to_string(&out).unwrap();
                        if to_ws_tx.send(Message::Text(s.into())).is_err() {
                            break;
                        }
                    }
                }

                Ok(ClientMsg::JoinMatch { match_id }) => {
                    info!(%match_id, "join_match parsed");

                    if player_id.is_none() || display_name.is_none() {
                        let out = ServerMsg::Error {
                            code: "NO_PLAYER_ID".into(),
                            message: "Send join message first".into(),
                        };
                        let s = serde_json::to_string(&out).unwrap();
                        if to_ws_tx.send(Message::Text(s.into())).is_err() {
                            break;
                        }
                        continue;
                    }

                    let name = display_name.as_ref().unwrap().clone();
                    let pid = player_id.as_ref().unwrap().clone();

                    // Acquire write lock, add O, then subscribe this socket
                    let mut reg = registry.write().await;
                    if let Some(room) = reg.get_mut(&match_id) {
                        if room.state.players.len() >= 2 {
                            let out = ServerMsg::Error {
                                code: "MATCH_FULL".into(),
                                message: "Match already has 2 players".into(),
                            };
                            let s = serde_json::to_string(&out).unwrap();
                            if to_ws_tx.send(Message::Text(s.into())).is_err() {
                                break;
                            }
                            continue;
                        }

                        // Add player O to authoritative state
                        let you = push_player(room, pid.clone(), name.clone(), "O");

                        // Ack only to the joiner
                        let out = ServerMsg::JoinedMatch {
                            match_id: match_id.clone(),
                            you,
                        };
                        let s = serde_json::to_string(&out).unwrap();
                        if to_ws_tx.send(Message::Text(s.into())).is_err() {
                            break;
                        }

                        // Subscribe this socket to the room’s broadcast
                        let rx = room.bcast.subscribe();
                        drop(reg); // release lock before awaits

                        // Broadcast updated state to ALL sockets (A will see B)
                        let _ = broadcast_state(&registry, &match_id).await;

                        // Start forwarding broadcasts → this socket if not already started
                        if bcast_forwarder.is_none() {
                            if let (Some(ws_tx), Some(to_ws_rx)) = (ws_tx_opt.take(), to_ws_rx_opt.take()) {
                                bcast_forwarder = Some(spawn_broadcast_forwarder(rx, ws_tx, to_ws_rx));
                            }
                        }

                        info!(%match_id, player_id=%pid, "player joined; state broadcasted");
                    } else {
                        drop(reg);
                        let out = ServerMsg::Error {
                            code: "MATCH_NOT_FOUND".into(),
                            message: format!("Match {} not found", match_id),
                        };
                        let s = serde_json::to_string(&out).unwrap();
                        if to_ws_tx.send(Message::Text(s.into())).is_err() {
                            break;
                        }
                    }
                }

                Err(e) => {
                    warn!(error = %e, "bad json");
                    let out = ServerMsg::Error {
                        code: "BAD_JSON".into(),
                        message: e.to_string(),
                    };
                    let s = serde_json::to_string(&out).unwrap();
                    if to_ws_tx.send(Message::Text(s.into())).is_err() {
                        break;
                    }
                }
            }
        }
    }

    if let Some(task) = bcast_forwarder {
        task.abort();
    }
    info!("websocket closed");
}

/// Forwards room broadcasts and app-initiated messages → this socket.
/// Each connection gets exactly one of these.
fn spawn_broadcast_forwarder(
    mut rx: tokio::sync::broadcast::Receiver<ServerMsg>,
    mut ws_tx: SplitSink<WebSocket, Message>,
    mut to_ws_rx: mpsc::UnboundedReceiver<Message>,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        loop {
            tokio::select! {
                // Messages the handler wants to push to the client (acks, errors, etc.)
                Some(msg) = to_ws_rx.recv() => {
                    if ws_tx.send(msg).await.is_err() { break; }
                }
                // Broadcast messages from the room
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