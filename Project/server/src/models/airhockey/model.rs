use std::collections::HashMap;
use crate::models::airhockey::{
    collision::{CollisionType, broad_phase, narrow_phase, reflect_wall, reflect_paddle},
    table::Table,
};
use crate::types::Vector2;

#[derive(Debug, Clone)]
pub struct AirHockeyModel {
    pub table: Table,
    pub players: HashMap<u8, String>,
}

impl AirHockeyModel {
    /// Create a new AirHockeyModel with a fresh table and two paddles
    pub fn new(player1: String, player2: String) -> Self {
        let table = Table::new();
        let mut players = HashMap::new();
        players.insert(1, player1);
        players.insert(2, player2);

        // Paddles are already created in Table::new with player numbers 1 and 2
        Self { 
            table,
            players, 
        }
    }

    /// Apply input to a paddle (player_number = 1 or 2)
    pub fn update_paddle(&mut self, player_number: u8, position: Option<Vector2>, velocity: Option<Vector2>) {
        self.table.apply_paddle_input(player_number, position, velocity);
    }

    /// Tick: advance the game state by dt seconds
    pub fn tick(&mut self, dt: f32) {
        // Move puck
        self.table.update(dt);

        // Broad-phase collision check
        if broad_phase(&self.table.puck, &self.table, &self.table.paddles) {
            let collision = narrow_phase(&self.table.puck, &self.table);

            match collision {
                CollisionType::Wall => reflect_wall(&mut self.table.puck, self.table.width.clone(), self.table.height.clone()),
                CollisionType::Paddle(player_number) => {
                    if let Some(paddle) = self.table.paddles.get(&player_number) {
                        reflect_paddle(&mut self.table.puck, paddle);
                    }
                }
                CollisionType::Goal(player_number) => {
                    if let Some(score) = self.table.score.get_mut(&player_number) {
                        *score += 1;
                    }
                    self.table.reset_puck();
                }
                CollisionType::None => {}
            }
        }
    }
}
