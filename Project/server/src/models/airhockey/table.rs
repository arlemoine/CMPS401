use std::collections::HashMap;

use crate::{
    models::airhockey::{paddle::Paddle, puck::Puck},
    types::Vector2,
};

#[derive(Debug, Clone)]
pub enum GameEvent {
    Update,
    Score { scoring_player: u8 },
    GameOver { winner: u8 },
}

#[derive(Debug, Clone)]
pub struct Table {
    // Board configs
    pub width: f32,
    pub height: f32,

    // Spawn locations
    pub p1_spawn: Vector2,
    pub p2_spawn: Vector2,
    pub puck_spawn: Vector2,

    // Resource setup
    pub puck: Puck,
    pub paddles: HashMap<u8, Paddle>, // player number -> paddle
    pub score: HashMap<u8, u8>,       // player number -> score
}

impl Table {
    /// Create a new Table with two players (player 1 = bottom, player 2 = top)
    pub fn new() -> Self {
        // Board configs
        let width = 400.0;
        let height = 800.0;
        let mid_x = width / 2.0;
        let mid_y = height / 2.0;

        // Spawn locations
        let spawn_gap = 40.0;
        let p1_y = spawn_gap;
        let p2_y = height - spawn_gap;
        let p1_spawn = Vector2 { x: mid_x, y: p1_y };
        let p2_spawn = Vector2 { x: mid_x, y: p2_y };
        let puck_spawn = Vector2 { x: mid_x, y: mid_y };

        // Resource setup
        let puck: Puck = Puck::new(puck_spawn.clone());
        let mut paddles = HashMap::new();
        paddles.insert(1, Paddle::new(1, p1_spawn.clone()));
        paddles.insert(2, Paddle::new(2, p2_spawn.clone()));

        let mut score = HashMap::new();
        score.insert(1, 0);
        score.insert(2, 0);

        Self {
            width,
            height,
            p1_spawn,
            p2_spawn,
            puck_spawn,
            puck,
            paddles,
            score,
        }
    }

    /// Reset puck to center and optionally reset velocities
    pub fn update(&mut self, dt: f32) -> Option<GameEvent> {
        // Update puck
        self.puck.update_position(dt);

        // Update paddles
        self.update_paddles(dt);

        // TODO: collision handling goes here

        // Check scoring
        if let Some(scoring_player) = self.check_goal() {
            if let Some(score) = self.score.get_mut(&scoring_player) {
                *score += 1;
            }
            self.reset_puck();
            return Some(GameEvent::Score { scoring_player });
        }

        Some(GameEvent::Update)
    }

    /// Returns Some(player_number) if a goal is scored
    fn check_goal(&self) -> Option<u8> {
        // If puck hits the top, player 1 scores
        if self.puck.position.y - self.puck.radius <= 0.0 {
            return Some(1);
        }
        // If puck hits the bottom, player 2 scores
        else if self.puck.position.y + self.puck.radius >= self.height {
            return Some(2);
        }
        None
    }

    /// Apply a paddle movement input (player_number = 1 or 2)
    pub fn apply_paddle_input(&mut self, player_number: u8, position: Option<Vector2>, velocity: Option<Vector2>) {
        if let Some(paddle) = self.paddles.get_mut(&player_number) {
            if let Some(pos) = position {
                paddle.set_position(pos);
            }
            if let Some(vel) = velocity {
                paddle.set_velocity(vel.x, vel.y);
            }
        }
    }

    pub fn update_paddles(&mut self, dt: f32) {
        for paddle in self.paddles.values_mut() {
            // Update paddle position
            paddle.update(dt);

            // Clamp to table bounds
            paddle.position.x = paddle.position.x.clamp(paddle.radius, self.width - paddle.radius);
            paddle.position.y = paddle.position.y.clamp(paddle.radius, self.height - paddle.radius);
        }
    }

    pub fn reset_puck(&mut self) {
        self.puck.position.x = self.width / 2.0;
        self.puck.position.y = self.height / 2.0;
    }
}
