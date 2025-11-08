#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RpsChoice {
    Rock,
    Paper,
    Scissors,
}

impl RpsChoice {
    pub fn from_str(choice: &str) -> Option<Self> {
        match choice.trim().to_lowercase().as_str() {
            "rock" | "r" => Some(Self::Rock),
            "paper" | "p" => Some(Self::Paper),
            "scissors" | "s" => Some(Self::Scissors),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Rock => "rock",
            Self::Paper => "paper",
            Self::Scissors => "scissors",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RpsRoundResult {
    Pending,
    Player1,
    Player2,
    Tie,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PlayerSlot {
    Player1,
    Player2,
}

#[derive(Debug, Clone)]
pub struct RockPaperScissorsModel {
    pub player1_name: Option<String>,
    pub player2_name: Option<String>,
    pub player1_choice: Option<RpsChoice>,
    pub player2_choice: Option<RpsChoice>,
    pub winner: RpsRoundResult,
}

impl RockPaperScissorsModel {
    pub fn new() -> Self {
        Self {
            player1_name: None,
            player2_name: None,
            player1_choice: None,
            player2_choice: None,
            winner: RpsRoundResult::Pending,
        }
    }

    pub fn reset_round(&mut self) {
        self.player1_choice = None;
        self.player2_choice = None;
        self.winner = RpsRoundResult::Pending;
    }

    pub fn both_players_joined(&self) -> bool {
        self.player1_name.is_some() && self.player2_name.is_some()
    }

    pub fn both_choices_made(&self) -> bool {
        self.player1_choice.is_some() && self.player2_choice.is_some()
    }

    pub fn submit_choice(&mut self, player_name: &str, choice: RpsChoice) -> Result<(), &'static str> {
        if self.winner != RpsRoundResult::Pending && self.both_choices_made() {
            // Automatically start the next round if both players already finished
            self.reset_round();
        }

        match self.player_slot(player_name) {
            Some(PlayerSlot::Player1) => {
                self.player1_choice = Some(choice);
                Ok(())
            }
            Some(PlayerSlot::Player2) => {
                self.player2_choice = Some(choice);
                Ok(())
            }
            None => Err("unknown_player"),
        }
    }

    pub fn resolve_round(&mut self) -> RpsRoundResult {
        if !self.both_choices_made() {
            self.winner = RpsRoundResult::Pending;
            return self.winner;
        }

        use RpsChoice::*;

        let p1 = self.player1_choice.unwrap();
        let p2 = self.player2_choice.unwrap();

        self.winner = if p1 == p2 {
            RpsRoundResult::Tie
        } else if (p1 == Rock && p2 == Scissors)
            || (p1 == Paper && p2 == Rock)
            || (p1 == Scissors && p2 == Paper)
        {
            RpsRoundResult::Player1
        } else {
            RpsRoundResult::Player2
        };

        self.winner
    }

    pub fn winner_name(&self) -> Option<&str> {
        match self.winner {
            RpsRoundResult::Player1 => self.player1_name.as_deref(),
            RpsRoundResult::Player2 => self.player2_name.as_deref(),
            _ => None,
        }
    }

    fn player_slot(&self, player_name: &str) -> Option<PlayerSlot> {
        if self.player1_name.as_deref() == Some(player_name) {
            Some(PlayerSlot::Player1)
        } else if self.player2_name.as_deref() == Some(player_name) {
            Some(PlayerSlot::Player2)
        } else {
            None
        }
    }
}
