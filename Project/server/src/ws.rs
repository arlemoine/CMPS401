use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
};
use futures::{StreamExt, SinkExt};
use std::sync::Arc;
use tokio::sync::mpsc;

use crate::models::appstate::AppState;
use crate::types::{ClientMessage, EchoPayload, ServerMessage};

use crate::routes::{
    echo_handler::echo_handler,
    gameroom_handler::gameroom_handler,
    chat_handler::chat_handler,
};

#[axum::debug_handler]
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

/// Parse client JSON → enum
fn parse_client_message(text: &str) -> Result<ClientMessage, String> {
    serde_json::from_str::<ClientMessage>(text)
        .map_err(|_| "Invalid JSON for ClientMessage".into())
}

/// Broadcast to all clients in room
async fn broadcast_to_room(msg: ServerMessage, state: &Arc<AppState>) {
    // Determine game_id
    let game_id = match &msg {
        ServerMessage::GameRoom(payload) => &payload.game_id,
        ServerMessage::Chat(payload) => &payload.game_id,
        _ => return,
    };

    let mut rooms = state.rooms.write().await;

    if let Some(room) = rooms.get_mut(game_id) {
        let serialized = serde_json::to_string(&msg).unwrap();

        // Remove any senders that fail
        room.txs.retain(|tx| tx.send(Message::Text(serialized.clone().into())).is_ok());
    }
}

/// Handle the WebSocket connection
pub async fn handle_socket(socket: WebSocket, state: Arc<AppState>) {
    // Create a channel to send messages TO this client
    let (tx, mut rx) = mpsc::unbounded_channel::<Message>();

    // Clone for room storage
    let tx_for_state = tx.clone();

    // Split to read client messages
    let (mut ws_tx, mut ws_rx) = socket.split();

    // Spawn task: forward messages from channel → websocket
    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            let _ = ws_tx.send(msg).await;
        }
    });

    // Main loop: read client messages
    while let Some(Ok(msg)) = ws_rx.next().await {
        if let Message::Text(text) = msg {
            let parsed = parse_client_message(&text);

            match parsed {
                Ok(client_msg) => match client_msg {
                    ClientMessage::GameRoom(payload) => {
                        let response = gameroom_handler(payload, &state, tx_for_state.clone()).await;
                        broadcast_to_room(response, &state).await;
                    }
                    ClientMessage::Echo(payload) => {
                        let response = echo_handler(payload, &state);
                        let serialized = serde_json::to_string(&response).unwrap();
                        let _ = tx_for_state.send(Message::Text(serialized.into()));
                    }
                    ClientMessage::Chat(payload) => {
                        let response = chat_handler(payload, &state).await;
                        broadcast_to_room(response, &state).await;
                    }
                    _ => {}
                },

                Err(err_str) => {
                    let error_payload = ServerMessage::Echo(EchoPayload { message: err_str });
                    let serialized = serde_json::to_string(&error_payload).unwrap();
                    let _ = tx_for_state.send(Message::Text(serialized.into()));
                }
            }
        }
    }

    // On disconnect: remove tx from any room it belonged to
    cleanup_sender(&state, &tx_for_state).await;
}

/// Remove tx from all rooms when client disconnects
async fn cleanup_sender(state: &Arc<AppState>, tx: &mpsc::UnboundedSender<Message>) {
    let mut rooms = state.rooms.write().await;

    for room in rooms.values_mut() {
        room.txs.retain(|t| !t.same_channel(tx));
    }
}
