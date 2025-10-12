use crate::types::{MatchState, Player, ServerMsg};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{broadcast, RwLock};

/// A live room: authoritative state + broadcast bus for all sockets in the room.
pub struct MatchRoom {
    pub state: MatchState,
    pub bcast: broadcast::Sender<ServerMsg>,
}

pub type MatchRegistry = Arc<RwLock<HashMap<String, MatchRoom>>>;

pub fn new_match_id() -> String {
    use rand::Rng;
    let mut rng = rand::rng();
    (0..4).map(|_| rng.random_range(b'A'..=b'Z') as char).collect()
}

/// Create a room with a fresh broadcast channel and return a receiver for the creator.
pub async fn create_room(
    registry: &MatchRegistry,
    state: MatchState,
) -> (String, broadcast::Receiver<ServerMsg>) {
    let (tx, rx) = broadcast::channel::<ServerMsg>(64);
    let id = state.match_id.clone();
    registry
        .write()
        .await
        .insert(id.clone(), MatchRoom { state, bcast: tx });
    (id, rx)
}

/// Subscribe a socket to a room’s broadcast stream.
pub async fn subscribe(
    registry: &MatchRegistry,
    match_id: &str,
) -> Option<broadcast::Receiver<ServerMsg>> {
    let map = registry.read().await;
    map.get(match_id).map(|room| room.bcast.subscribe())
}

/// Broadcast the current state snapshot to *all* sockets in the room.
pub async fn broadcast_state(registry: &MatchRegistry, match_id: &str) -> Option<()> {
    let map = registry.read().await;
    let room = map.get(match_id)?;
    let update = ServerMsg::StateUpdate {
        match_id: room.state.match_id.clone(),
        players: room.state.players.clone(),
        status: room.state.status.clone(),
    };
    // Ignore send errors (e.g., no subscribers) — this is fine.
    let _ = room.bcast.send(update);
    Some(())
}

/// Small helper to push a player into a room’s state with the given mark.
/// Returns the Player that was added.
pub fn push_player(room: &mut MatchRoom, id: String, display_name: String, mark: &str) -> Player {
    let p = Player {
        id,
        display_name,
        mark: mark.to_string(),
    };
    room.state.players.push(p.clone());
    if room.state.players.len() == 2 {
        room.state.status = "IN_PROGRESS".to_string();
    }
    p
}