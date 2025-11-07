use chrono::Local;
use std::sync::Arc;

use crate::models::appstate::AppState;
use crate::types::{ChatPayload, ServerMessage};

fn get_timestamp() -> String {
    Local::now().format("%I:%M %p").to_string()
}

pub async fn chat_handler(
    payload: ChatPayload,
    _state: &Arc<AppState>
) -> ServerMessage {
    let mut response = payload.clone();
    response.time = get_timestamp();
    ServerMessage::Chat(response)
}
