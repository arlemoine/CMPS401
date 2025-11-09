use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
  AppState,
  models::{gameroom::{GameRoom, GameType}, uno::model::*},
  types::{UnoPayloadToServer, UnoPayloadToClient, ServerMessage},
};

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

    // If there is a pending draw, enforce at start of turn
    if s.started && s.pending_draw > 0 {
        if s.current_player() == Some(&payload.player_name) && payload.action != "draw_card" && payload.action != "pass_turn" && payload.action != "play_card" {
            // we only enforce inside play path; leave as-is if action is unrelated
        }
    }

    match payload.action.as_str() {
        "start" => {
            s.start();
        }

        "draw_card" => {
            if let Some(c) = s.deck.pop() {
                if let Some(hand) = s.hand_of_mut(&payload.player_name) {
                    hand.push(c);
                }
            }
            // no auto-play in MVP
        }

        "pass_turn" => {
            // if pending draw > 0, force draw and skip was already applied at turn start via s.force_draw_and_skip()
            s.advance_turn(1);
        }

        "play_card" => {
            if let (Some(top), Some(card)) = (s.discard_top.clone(), payload.card.clone()) {
                // Validate playable (classic: match color OR rank OR any wild)
                let ok = UnoModel::can_play_on_top(&top, &card);

                if ok {
                    // Remove from hand (first matching)
                    if let Some(hand) = s.hand_of_mut(&payload.player_name) {
                        if let Some(pos) = hand.iter().position(|c| c == &card) {
                            hand.remove(pos);
                        } else {
                            // no-op if client lied
                        }
                    }

                    match card.rank {
                        UnoRank::Skip => s.apply_skip(card),
                        UnoRank::Reverse => s.apply_reverse(card),
                        UnoRank::DrawTwo => s.apply_draw_two(card),
                        UnoRank::Wild => {
                            let chosen = parse_choose(&payload.choose_color);
                            if let Some(c) = chosen { s.apply_wild(card, c); } else { /* no-op */ }
                        }
                        UnoRank::WildDrawFour => {
                            let chosen = parse_choose(&payload.choose_color);
                            if let Some(c) = chosen { s.apply_wild_draw_four(card, c); } else { /* no-op if missing */ }
                        }
                        _ => s.apply_number_play(card),
                    }

                    // UNO win condition
                    if let Some(hand) = s.hands.get(&payload.player_name) {
                        if hand.is_empty() {
                            s.winner = Some(payload.player_name.clone());
                        }
                    }
                }
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

// --- helpers the ws loop can also call ---

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