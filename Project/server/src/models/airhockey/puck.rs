use crate::models::airhockey::config::*;
use crate::types::Vector2;

#[derive(Debug, Clone)]
pub struct Puck {
    pub position: Vector2,
    pub velocity: Vector2,
    pub radius: f32,
    pub max_speed: f32,
}

impl Puck {
    /// Create a new puck at the center of the table with zero velocity
    pub fn new(start_position: Vector2) -> Self {
        let position = start_position;
        let velocity = Vector2 { x: 0.0, y: 0.0 };
        let radius = 10.0;
        let max_speed = PUCK_MAX_SPEED;
        
        Self {
            position,
            velocity,
            radius,
            max_speed,
        }
    }

    /// Manually move puck to a position (e.g., teleporting or snapping)
    pub fn set_position(&mut self, pos: Vector2) {
        self.position = pos;
    }

    /// Reset puck velocity to 0
    pub fn reset_velocity(&mut self) {
        self.velocity.x = 0.0;
        self.velocity.y = 0.0;
    }

    /// Update puck position based on its velocity and elapsed time `dt` (seconds)
    pub fn update_position(&mut self, dt: f32) {
        self.position.x += self.velocity.x * dt;
        self.position.y += self.velocity.y * dt;
    }

    /// Apply friction to gradually slow down the puck
    pub fn apply_friction(&mut self, friction_coefficient: f32, dt: f32) {
        self.velocity.x -= self.velocity.x * friction_coefficient * dt;
        self.velocity.y -= self.velocity.y * friction_coefficient * dt;
    }

    /// Clamp velocity to max speed
    pub fn clamp_velocity(&mut self) {
        let speed = (self.velocity.x.powi(2) + self.velocity.y.powi(2)).sqrt();
        if speed > self.max_speed {
            let scale = self.max_speed / speed;
            self.velocity.x *= scale;
            self.velocity.y *= scale;
        }
    }

    /// Manually set velocity (used after collisions)
    pub fn set_velocity(&mut self, vx: f32, vy: f32) {
        self.velocity.x = vx;
        self.velocity.y = vy;
        self.clamp_velocity();
    }

    /// Helper: check if puck is moving
    pub fn is_moving(&self) -> bool {
        self.velocity.x.abs() > 0.01 || self.velocity.y.abs() > 0.01
    }
}
