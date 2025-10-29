use axum::extract::ws::{Message};
use std::sync::Arc;

use crate::appstate::AppState;
use crate::types::ServerMessage;

/// Echo handler: Send the same message back to the client
pub async fn echo_handler(
    message: String,
    _state: &Arc<AppState>,
    socket: &mut axum::extract::ws::WebSocket,
) {
    // Wrap the message in ServerMessage enum
    let server_msg = ServerMessage::Echo { message };

    // Serialize to JSON
    if let Ok(json_str) = serde_json::to_string(&server_msg) {
        let _ = socket.send(Message::Text(json_str.into())).await;
    }
}
