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
}

impl TicTacToeModel {
    pub fn new() -> Self {
        Self {
            board: [[0; 3]; 3],
            whos_turn: Player::Player1,
            winner: GameWinner::Pending,
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
