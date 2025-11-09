use tokio::sync::mpsc::UnboundedSender;
use axum::extract::ws::Message;

use crate::models::{
    rockpaperscissors::model::RockPaperScissorsModel,
    tictactoe::model::TicTacToeModel,
};

#[derive(Debug)]
pub enum GameType {
    TicTacToe(TicTacToeModel),
    RockPaperScissors(RockPaperScissorsModel),
    // List other game types here
}

/// A game room that holds users, chat, and a game of type T.
#[derive(Debug)]
pub struct GameRoom {
    pub game_id: String,
    pub users: Vec<String>,
    pub txs: Vec<UnboundedSender<Message>>, // transmitters for all members
    pub chat_log: Vec<String>,
    pub game: GameType, // the actual game model
}

impl GameRoom {
    /// Create a new game room with a specific game model
    pub fn new(game_id: String, game: GameType) -> Self {
        Self {
            game_id,
            users: Vec::new(),
            txs: Vec::new(),
            chat_log: Vec::new(),
            game,
        }
    }
}
