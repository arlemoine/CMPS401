use serde::{Deserialize, Serialize};
use rand::{seq::SliceRandom, thread_rng};
use std::collections::HashMap;

pub type PlayerId = String;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "PascalCase")]
pub enum UnoColor { Red, Yellow, Green, Blue, Wild }

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "PascalCase")]
pub enum UnoRank {
    #[serde(rename="0")] N0, #[serde(rename="1")] N1, #[serde(rename="2")] N2, #[serde(rename="3")] N3, #[serde(rename="4")] N4,
    #[serde(rename="5")] N5, #[serde(rename="6")] N6, #[serde(rename="7")] N7, #[serde(rename="8")] N8, #[serde(rename="9")] N9,
    Skip, Reverse, DrawTwo, Wild, WildDrawFour
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct UnoCard {
    pub color: UnoColor,
    pub rank: UnoRank,
}

#[derive(Clone, Debug, Default)]
pub struct UnoModel {
    pub players: Vec<PlayerId>,                      // seat order (names)
    pub current_idx: usize,                          // whose turn (index into players)
    pub direction: i8,                               // 1 or -1
    pub deck: Vec<UnoCard>,                          // face-down draw pile (top = last)
    pub discard_top: Option<UnoCard>,                // top of discard pile               
    pub chosen_color: Option<UnoColor>,             // UI hint chosen on Wild/WDF (non-binding; does NOT restrict rank plays)
    pub pending_draw: u8,                            // accumulated penalty
    pub hands: HashMap<PlayerId, Vec<UnoCard>>,      // hidden state per player
    pub winner: Option<PlayerId>,
    pub started: bool,
}

impl UnoModel {
    pub fn new() -> Self {
        Self { direction: 1, ..Default::default() }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn add_player(&mut self, name: &str) {
        if !self.players.contains(&name.to_string()) {
            self.players.push(name.to_string());
            self.hands.entry(name.to_string()).or_default();
        }
    }

    pub fn start(&mut self) {
        self.started = true;
        self.current_idx = 0;
        self.direction = 1;
        self.pending_draw = 0;
        self.chosen_color = None;
        self.deck = build_full_uno_deck();
        self.deck.shuffle(&mut thread_rng());

        // deal 7 to each
        for p in &self.players {
            let mut hand = Vec::with_capacity(7);
            for _ in 0..7 { if let Some(c) = self.deck.pop() { hand.push(c); } }
            self.hands.insert(p.clone(), hand);
        }

        // flip first non-wild to discard_top
        loop {
            if let Some(card) = self.deck.pop() {
                match card.rank {
                    UnoRank::Wild | UnoRank::WildDrawFour => {
                        // put it back somewhere and continue; simplest: push front of deck vector base
                        // (for MVP we just reinsert at position 0)
                        self.deck.insert(0, card);
                        continue;
                    }
                    _ => {
                        self.discard_top = Some(card);
                        break;
                    }
                }
            } else {
                break;
            }
        }
    }

    pub fn current_player(&self) -> Option<&PlayerId> {
        self.players.get(self.current_idx)
    }

    pub fn hand_of_mut(&mut self, player: &str) -> Option<&mut Vec<UnoCard>> {
        self.hands.get_mut(player)
    }

    pub fn public_counts(&self) -> Vec<u8> {
        self.players.iter()
            .map(|p| self.hands.get(p).map(|h| h.len() as u8).unwrap_or(0))
            .collect()
    }

    // Classic rule (no color lock): match color OR rank, or play any wild at any time
    pub fn can_play_on_top(top: &UnoCard, card: &UnoCard) -> bool {
        matches!(card.rank, UnoRank::Wild | UnoRank::WildDrawFour)
            || card.color == top.color
            || card.rank == top.rank
    }

    pub fn apply_number_play(&mut self, card: UnoCard) {
        self.discard_top = Some(card);
        self.advance_turn(1);
    }

    pub fn apply_skip(&mut self, card: UnoCard) {
        self.discard_top = Some(card);
        self.advance_turn(2);
    }

    pub fn apply_reverse(&mut self, card: UnoCard) {
        self.discard_top = Some(card);
        if self.players.len() == 2 {
            // reverse acts like skip in 2-player
            self.advance_turn(2);
        } else {
            self.direction *= -1;
            self.advance_turn(1);
        }
    }

    pub fn apply_draw_two(&mut self, card: UnoCard) {
        self.discard_top = Some(card);
        self.pending_draw = self.pending_draw.saturating_add(2);
        self.advance_turn(1);
    }

    pub fn apply_wild(&mut self, card: UnoCard, chosen: UnoColor) {
        self.discard_top = Some(card);
        self.chosen_color = Some(chosen); // non-binding UI hint
        self.advance_turn(1);
    }

    pub fn apply_wild_draw_four(&mut self, card: UnoCard, chosen: UnoColor) {
        self.discard_top = Some(card);
        self.chosen_color = Some(chosen); // non-binding UI hint
        self.pending_draw = self.pending_draw.saturating_add(4);
        self.advance_turn(1);
    }

    /// Called at the start of the current player's turn if pending_draw > 0.
    pub fn force_draw_and_skip(&mut self) {
        if self.pending_draw == 0 { return; }
        if let Some(p) = self.current_player().cloned() {
            let draw_n = self.pending_draw as usize;
            let drawn = draw_from_deck(&mut self.deck, draw_n);
            if let Some(hand) = self.hands.get_mut(&p) {
                hand.extend(drawn);
            }
        }
        self.pending_draw = 0;
        self.advance_turn(1);
    }

    pub fn advance_turn(&mut self, steps: usize) {
        let n = self.players.len();
        if n == 0 { return; }
        let dir = if self.direction >= 0 { 1isize } else { -1isize };
        let mut idx = self.current_idx as isize;
        for _ in 0..steps {
            idx = (idx + dir).rem_euclid(n as isize);
        }
        self.current_idx = idx as usize;
    }
}

// --------- deck utils ---------

fn build_full_uno_deck() -> Vec<UnoCard> {
    let mut deck = Vec::with_capacity(108);

    // Colored number/action cards
    let colors = [UnoColor::Red, UnoColor::Yellow, UnoColor::Green, UnoColor::Blue];
    let numbers = [
        UnoRank::N0, UnoRank::N1, UnoRank::N2, UnoRank::N3, UnoRank::N4,
        UnoRank::N5, UnoRank::N6, UnoRank::N7, UnoRank::N8, UnoRank::N9
    ];

    for color in colors {
        // one 0 per color
        deck.push(UnoCard { color: color.clone(), rank: UnoRank::N0 });
        // two of 1..9 per color
        for r in &numbers[1..] {
            deck.push(UnoCard { color: color.clone(), rank: r.clone() });
            deck.push(UnoCard { color: color.clone(), rank: r.clone() });
        }
        // two Skip, two Reverse, two DrawTwo per color
        for _ in 0..2 {
            deck.push(UnoCard { color: color.clone(), rank: UnoRank::Skip });
            deck.push(UnoCard { color: color.clone(), rank: UnoRank::Reverse });
            deck.push(UnoCard { color: color.clone(), rank: UnoRank::DrawTwo });
        }
    }

    // Wilds
    for _ in 0..4 {
        deck.push(UnoCard { color: UnoColor::Wild, rank: UnoRank::Wild });
        deck.push(UnoCard { color: UnoColor::Wild, rank: UnoRank::WildDrawFour });
    }
    deck
}

fn draw_from_deck(deck: &mut Vec<UnoCard>, n: usize) -> Vec<UnoCard> {
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        if let Some(c) = deck.pop() { out.push(c) }
    }
    out
}