use std::sync::Arc;

use crate::models::appstate::AppState;
use crate::types::{
        EchoPayload,
        ServerMessage,
};

/// Echo handler: just prepare the message to send back
pub fn echo_handler(
    payload: EchoPayload,
    _state: &Arc<AppState>,
) -> ServerMessage {
    ServerMessage::Echo(payload)
}
