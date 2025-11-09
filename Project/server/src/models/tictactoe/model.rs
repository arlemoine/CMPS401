#[derive(Debug, Clone, PartialEq)]
pub enum Player {
    Player1,
    Player2,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GameWinner {
    Pending,
    Player1,
    Player2,
    Tie,
}

#[derive(Debug, Clone)]
pub struct TicTacToeModel {
    pub board: [[i8; 3]; 3],
    pub whos_turn: Player,
    pub winner: GameWinner,
    pub player1_name: Option<String>, // ✅ ADDED: Track player 1's actual name
    pub player2_name: Option<String>, // ✅ ADDED: Track player 2's actual name
}

impl TicTacToeModel {
    pub fn new() -> Self {
        Self {
            board: [[0; 3]; 3],
            whos_turn: Player::Player1,
            winner: GameWinner::Pending,
            player1_name: None, // ✅ ADDED
            player2_name: None, // ✅ ADDED
        }
    }

pub fn assign_player(&mut self, player_name: String) -> Option<Player> {
        if self.player1_name.is_none() {
            self.player1_name = Some(player_name);
            Some(Player::Player1)
        } else if self.player2_name.is_none() {
            self.player2_name = Some(player_name);
            Some(Player::Player2)
        } else {
            None // Room is full
        }
    }

    // ✅ ADDED: Get player enum from name
    pub fn get_player_from_name(&self, player_name: &str) -> Option<Player> {
        if self.player1_name.as_deref() == Some(player_name) {
            Some(Player::Player1)
        } else if self.player2_name.as_deref() == Some(player_name) {
            Some(Player::Player2)
        } else {
            None
        }
    }

    // ✅ ADDED: Get name of current turn player
    pub fn current_player_name(&self) -> Option<&str> {
        match self.whos_turn {
            Player::Player1 => self.player1_name.as_deref(),
            Player::Player2 => self.player2_name.as_deref(),
        }
    }

    // ✅ ADDED: Get winner's name
    pub fn winner_name(&self) -> Option<&str> {
        match self.winner {
            GameWinner::Player1 => self.player1_name.as_deref(),
            GameWinner::Player2 => self.player2_name.as_deref(),
            _ => None,
        }
    }


    // Validate that the move is allowed
    pub fn validate_choice(&self, row: usize, col: usize) -> bool {
        self.board[row][col] == 0
    }

    // Place mark on the board
    pub fn mark_spot(&mut self, row: usize, col: usize) {
        let val = if self.whos_turn == Player::Player1 { 1 } else { -1 };
        self.board[row][col] = val;
    }

    // Switch to next turn
    pub fn next_turn(&mut self) {
        self.whos_turn = match self.whos_turn {
            Player::Player1 => Player::Player2,
            Player::Player2 => Player::Player1,
        };
    }

    // Check for winner or tie
    pub fn check_winner(&mut self) {
        let lines = [
            // rows
            [(0, 0), (0, 1), (0, 2)],
            [(1, 0), (1, 1), (1, 2)],
            [(2, 0), (2, 1), (2, 2)],
            // columns
            [(0, 0), (1, 0), (2, 0)],
            [(0, 1), (1, 1), (2, 1)],
            [(0, 2), (1, 2), (2, 2)],
            // diagonals
            [(0, 0), (1, 1), (2, 2)],
            [(0, 2), (1, 1), (2, 0)],
        ];

        for line in lines {
            let sum: i8 = line.iter().map(|&(r, c)| self.board[r][c]).sum();
            if sum == 3 {
                self.winner = GameWinner::Player1;
                return;
            } else if sum == -3 {
                self.winner = GameWinner::Player2;
                return;
            }
        }

        if self.board.iter().all(|row| row.iter().all(|&v| v != 0)) {
            self.winner = GameWinner::Tie;
        }
    }
}
