use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
  AppState,
  models::{gameroom::{GameRoom, GameType}, uno::model::*},
  types::{UnoPayloadToServer, UnoPayloadToClient, ServerMessage},
};

use serde_json;
use axum::extract::ws::Message;

pub async fn uno_handler(
    payload: UnoPayloadToServer,
    app_state: &Arc<AppState>,
    _current_room: Arc<RwLock<Option<String>>>,
) -> ServerMessage  {
    let mut rooms = app_state.rooms.write().await;
    let Some(room) = rooms.get_mut(&payload.game_id) else {
        // Room not found: return a neutral broadcast (FE can decide UX)
        return ServerMessage::Uno(UnoPayloadToClient {
            game_id: payload.game_id,
            players: None, current_idx: None, direction: None,
            top_discard: None, chosen_color: None, pending_draw: None,
            public_counts: None, hand: None, winner: None,
        });
    };

    let GameType::Uno(ref mut s) = room.game else {
        // wrong game type, send no-op state
        return public_snapshot_empty(payload.game_id);
    };

    // Auto-enforce pending draw penalties at the start of the current player's turn.
    // If this applies, short-circuit and broadcast updated state (no other action this turn).
    if s.is_players_turn(&payload.player_name) && s.enforce_pending_at_turn_start() {
        return build_public_update(&payload.game_id, s);
    }

    match payload.action.as_str() {
        "start" => {
            s.start();
        }

        "draw_card" => {
            if !s.is_players_turn(&payload.player_name) {
                return build_public_update(&payload.game_id, s);
            }
            let _ = s.draw_one(&payload.player_name);
        }

        "pass_turn" => {
            if !s.is_players_turn(&payload.player_name) {
                return build_public_update(&payload.game_id, s);
            }
            s.advance_turn(1);
        }

        "play_card" => {
            if !s.is_players_turn(&payload.player_name) {
                return build_public_update(&payload.game_id, s);
            }

            let Some(card) = payload.card.as_ref() else {
                return build_public_update(&payload.game_id, s);
            };

            // Parse chosen color for Wild/WDF (non-binding hint semantics)
            let choose = match card.rank {
                UnoRank::Wild | UnoRank::WildDrawFour => parse_choose(&payload.choose_color),
                _ => None,
            };

            if let Err(_e) = s.play_card_tx(&payload.player_name, card, choose) {
                // For now, ignore error details; simply return current state. (Task #8 will map errors.)
                return build_public_update(&payload.game_id, s);
            }
        }

        "call_uno" => {
            // MVP: handle as “mark self safe” or let opponents penalize if missed.
            // You can store a flag on state if you want a stricter timing rule.
        }

        "request_state" => {
            // no-op; just fall through to building updates
        }

        _ => { /* ignore unknown */ }
    }

    // Return a public snapshot; in ws loop, follow up with private hands per player.
    build_public_update(&payload.game_id, s)
}

fn parse_choose(s: &Option<String>) -> Option<UnoColor> {
    match s.as_deref() {
        Some("Red") => Some(UnoColor::Red),
        Some("Yellow") => Some(UnoColor::Yellow),
        Some("Green") => Some(UnoColor::Green),
        Some("Blue") => Some(UnoColor::Blue),
        _ => None,
    }
}

pub fn build_public_update(game_id: &str, s: &UnoModel) -> ServerMessage {
    ServerMessage::Uno(UnoPayloadToClient {
        game_id: game_id.to_string(),
        players: Some(s.players.clone()),
        current_idx: Some(s.current_idx as i32),
        direction: Some(s.direction),
        top_discard: s.discard_top.clone(),
        chosen_color: s.chosen_color.as_ref().map(|c| match c {
            UnoColor::Red => "Red".to_string(),
            UnoColor::Yellow => "Yellow".to_string(),
            UnoColor::Green => "Green".to_string(),
            UnoColor::Blue => "Blue".to_string(),
            UnoColor::Wild => "Wild".to_string(),
        }),
        pending_draw: Some(s.pending_draw),
        public_counts: Some(s.public_counts()),
        hand: None, // public snapshot
        winner: s.winner.clone(),
    })
}

pub fn build_private_hand(game_id: &str, s: &UnoModel, player: &str) -> ServerMessage {
    let hand = s.hands.get(player).cloned().unwrap_or_default();
    ServerMessage::Uno(UnoPayloadToClient {
        game_id: game_id.to_string(),
        players: None, current_idx: None, direction: None,
        top_discard: None, chosen_color: None, pending_draw: None,
        public_counts: None,
        hand: Some(hand),
        winner: None,
    })
}

fn public_snapshot_empty(game_id: String) -> ServerMessage {
    ServerMessage::Uno(UnoPayloadToClient {
        game_id,
        players: None, current_idx: None, direction: None,
        top_discard: None, chosen_color: None, pending_draw: None,
        public_counts: None, hand: None, winner: None,
    })
}

/// Send each UNO player's private hand to their own socket for this room.
/// Lives in UNO route layer to keep ws.rs game-agnostic.
pub async fn dm_uno_private_hands(app_state: Arc<AppState>, game_id: &str) {
    let rooms = app_state.rooms.read().await;
    if let Some(room) = rooms.get(game_id) {
        if let GameType::Uno(ref model) = room.game {
            for (i, player) in model.players.iter().enumerate() {
                if let Some(tx) = room.txs.get(i) {
                    let hand_msg = build_private_hand(game_id, model, player);
                    if let Ok(text) = serde_json::to_string(&hand_msg) {
                        let _ = tx.send(Message::Text(text.into()));
                    }
                }
            }
        }
    }
}