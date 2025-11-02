use chrono::Local;
use std::sync::Arc;

use crate::types::{appstate::AppState, types::{ChatPayload, ServerMessage}};

fn get_timestamp() -> String {
    let now = Local::now();
    let time_string = now.format("%I:%M %p").to_string();
    time_string
}

pub async fn chat_handler(payload: ChatPayload, state: &Arc<AppState>) -> ServerMessage {
    let mut response = payload.clone();
    response.time = get_timestamp();
    ServerMessage::Chat(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timestamp() {
        let result: String = get_timestamp();
        println!("{}",result);
    }
}