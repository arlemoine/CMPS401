use axum::{
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
    chat::chat_handler,
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
                }
            }
        }
    }
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
            let response = join_game(payload, state).await;
            send_server_message(response, tx).await;
        }
        ClientMessage::Chat(payload) => {
            let response = chat_handler(payload, state).await;
            send_server_message(response, tx).await;
        }
        // You can add MovePiece, Chat, etc. here in the future
    }
}

/// Helper to serialize ServerMessage -> send over socket
async fn send_server_message(msg: ServerMessage, tx: &mut futures::stream::SplitSink<WebSocket, Message>) {
    if let Ok(json_str) = serde_json::to_string(&msg) {
        let _ = tx.send(Message::Text(json_str.into())).await;
    }
}
