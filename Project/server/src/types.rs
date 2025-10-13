// server/src/types.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub mark: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct MatchState {
    #[serde(rename = "matchId")]
    pub match_id: String,
    pub players: Vec<Player>,
    pub status: String, // "WAITING", "IN_PROGRESS", "FINISHED"

    pub board: Vec<Option<String>>, // 9 cells, X/O or None
    pub turn: Option<String>,       // whose mark is the current turn ("X"/"O")
}

impl MatchState {
    pub fn new(match_id: String, players: Vec<Player>) -> Self {
        Self {
            match_id,
            players,
            status: "WAITING".into(),
            board: vec![None; 9],
            turn: Some("X".into()), // X always starts
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum ClientMsg {
    #[serde(rename = "join")]
    Join {
        #[serde(rename = "displayName")]
        display_name: String,
    },

    #[serde(rename = "create_match")]
    CreateMatch {},

    #[serde(rename = "join_match")]
    JoinMatch {
        #[serde(rename = "matchId")]
        match_id: String,
    },

    #[serde(rename = "make_move")]
    MakeMove {
        #[serde(rename = "matchId")]
        match_id: String,
        index: usize,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "payload")]
pub enum ServerMsg {
    #[serde(rename = "hello")]
    Hello {
        #[serde(rename = "serverVersion")]
        server_version: &'static str,
    },

    #[serde(rename = "match_created")]
    MatchCreated {
        #[serde(rename = "matchId")]
        match_id: String,
        you: Player,
    },

    #[serde(rename = "error")]
    Error { code: String, message: String },

    #[serde(rename = "state_update")]
    StateUpdate {
        #[serde(rename = "matchId")]
        match_id: String,
        players: Vec<Player>,
        status: String,
        board: Vec<Option<String>>,
        turn: Option<String>,
    },

    #[serde(rename = "joined_match")]
    JoinedMatch {
        #[serde(rename = "matchId")]
        match_id: String,
        you: Player,
    },
}
