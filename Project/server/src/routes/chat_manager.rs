use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use axum::extract::ws::Message;
use tokio::sync::mpsc::UnboundedSender;


/// Tracks all active chat rooms and their participants (to be added later).
#[derive(Clone, Default)]
pub struct ChatManager {
    pub rooms: Arc<RwLock<HashMap<String, Vec<UnboundedSender<Message>>>>>,
}

impl ChatManager {
    /// Adds a user to a chat room.
    pub async fn join_room(&self, room: &str, sender: UnboundedSender<Message>) {
        let mut rooms = self.rooms.write().await;
        let entry = rooms.entry(room.to_string()).or_insert_with(Vec::new);
        entry.push(sender);
    }

    /// Broadcasts a message to all users in a room
    pub async fn broadcast_message(&self, room: &str, message: Message) {
        let rooms = self.rooms.read().await;
        if let Some(senders) = rooms.get(room) {
            for sender in senders {
                // Ignore send errors (client may have disconnected)
                let _ = sender.send(message.clone());
            }
        }
    }
}

