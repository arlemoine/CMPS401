use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path, State,
    },
    response::IntoResponse,
};
use futures_util::{SinkExt, StreamExt};
use serde_json;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use std::collections::HashMap;

use crate::{
    types::{ClientMsg, ServerMsg, Player},
    match_::Match,
};

// Define MatchRegistry type (same as in main.rs)
type MatchRegistry = Arc<Mutex<HashMap<String, Match>>>;

/// Handler for the HTTP upgrade to a WebSocket connection.
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Path((match_id, player_id)): Path<(String, String)>,
    State(registry): State<MatchRegistry>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, registry, match_id, player_id))
}

/// Core logic for handling the WebSocket connection.
pub async fn handle_socket(
    socket: WebSocket,
    registry: MatchRegistry,
    match_id: String,
    player_id: String,
) {
    // 1. Split the socket into sender and receiver
    let (mut client_ws_tx, mut client_ws_rx) = socket.split(); 

    // 2. Channel to receive messages broadcasted by the Match's GameLogic.
    let (tx_to_client, mut rx_from_match) = mpsc::unbounded_channel::<String>();

    // 3. Lock the match registry to find and join the match
    let mut registry_lock = registry.lock().await;
    let match_data = registry_lock.get_mut(&match_id);

    let match_instance = match match_data {
        Some(m) => m,
        None => {
            // Match not found, send an error and exit
            let error_msg = ServerMsg::Error {
                code: "MATCH_NOT_FOUND".to_string(),
                message: format!("Match {} does not exist.", match_id),
            };
            if let Ok(json_string) = serde_json::to_string(&error_msg) {
                let _ = client_ws_tx.send(Message::Text(json_string.into())).await;
            }
            return;
        }
    };
    
    // 4. Register the player with the Match instance (using player_id as display_name for now)
    let player_data = match_instance.add_player(player_id.clone(), player_id.clone()); 
    
    // 5. Send initial "JoinedMatch" message
    let initial_msg = ServerMsg::JoinedMatch {
        you: player_data.clone(), 
        match_id: match_id.clone(),
        body: match_instance.game.get_state(&match_id).get_body().clone(),
    };
    
    // Register the client's dedicated channel for receiving broadcasts from the Match.
    // This allows the GameLogic trait to broadcast to all clients connected to this match.
    let mut player_listeners = match_instance.game.get_listeners();
    player_listeners.insert(player_id.clone(), tx_to_client);

    // Release the registry lock before spawning long-lived tasks
    drop(registry_lock);

    if let Ok(json_string) = serde_json::to_string(&initial_msg) {
        if client_ws_tx.send(Message::Text(json_string.into())).await.is_err() { 
            println!("Error sending initial message to {}.", player_id);
            return; 
        }
    }

    // --- Message Forwarding Task (SENDER) ---
    // Reads messages from the match's channel (rx_from_match) and sends them to the client's WebSocket (client_ws_tx).
    let mut sender_task = tokio::spawn(async move {
        while let Some(json_string) = rx_from_match.recv().await {
            if client_ws_tx.send(Message::Text(json_string.into())).await.is_err() { 
                // Client disconnected
                break;
            }
        }
    });

    // --- Message Listening Task (RECEIVER) ---
    // Listens for messages from the client (client_ws_rx) and routes them to the Match.
    let match_id_clone = match_id.clone();
    let registry_clone = registry.clone();
    let player_id_clone = player_id.clone();

    let mut receiver_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = client_ws_rx.next().await {
            if let Message::Text(text) = msg {
                match serde_json::from_str::<ClientMsg>(&text) {
                    Ok(client_msg) => {
                        let mut registry_lock = registry_clone.lock().await;
                        if let Some(m) = registry_lock.get_mut(&match_id_clone) {
                            // Call the async handler method on the Match struct
                            m.handle_client_message(&player_id_clone, client_msg).await;
                        }
                    }
                    Err(e) => {
                        println!("Failed to parse client message from {}: {} - Error: {}", player_id_clone, text, e);
                    }
                }
            }
        }
    });

    // Wait for either the sender or receiver task to complete (meaning disconnection)
    tokio::select! {
        _ = &mut sender_task => {
            receiver_task.abort();
        },
        _ = &mut receiver_task => {
            sender_task.abort();
        },
    }

    // --- Connection Cleanup ---
    let mut registry_lock = registry.lock().await;
    if let Some(m) = registry_lock.get_mut(&match_id) {
        m.remove_player(&player_id);

        // Remove the client's listener channel
        m.game.get_listeners().remove(&player_id);

        // If the match is completely empty, remove it from the registry
        if m.players.len() == 0 && m.game.get_listeners().is_empty() {
            registry_lock.remove(&match_id);
            println!("Match {} closed and removed from registry.", match_id);
        }
    }
}
