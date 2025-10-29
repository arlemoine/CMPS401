use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path, State,
    },
    response::IntoResponse,
};
use futures::{sink::SinkExt, stream::StreamExt}; // for reading/writing WebSocket messages
use std::sync::Arc;

use crate::appstate::AppState;
use crate::gameroom::GameRoom;
use crate::routes::echo::echo_handler;
use crate::types::{ClientMessage, ServerMessage};

#[axum::debug_handler]
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

pub async fn handle_socket(mut socket: WebSocket, state: Arc<AppState>) {
    while let Some(Ok(msg)) = socket.next().await {
        match msg {
            Message::Text(text) => {
                // Deserialize into ClientMessage
                match serde_json::from_str::<ClientMessage>(&text) {
                    Ok(client_msg) => {
                        match client_msg {
                            ClientMessage::Echo { message } => {
                                // call your echo handler
                                echo_handler(message, &state, &mut socket).await;
                            }
                            // ClientMessage::JoinRoom { room_id } => {
                            //     join_room_handler(room_id, &state, &mut socket).await;
                            // }
                            // ClientMessage::MovePiece { from, to } => {
                            //     game_handler(from, to, &state, &mut socket).await;
                            // }
                            // ClientMessage::Chat { text } => {
                            //     chat_handler(text, &state, &mut socket).await;
                            // }
                        }
                    }
                    Err(_) => {
                        let _ = socket.send(Message::Text(
                            "Invalid JSON format for ClientMessage".into(),
                        ))
                        .await;
                    }
                }
            }
            Message::Close(_) => break, // client disconnected
            _ => {}
        }
    }
}
