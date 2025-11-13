use crate::models::airhockey::config::*;
use crate::types::Vector2;

#[derive(Debug, Clone)]
pub struct Paddle {
    pub player_num: u8,
    pub position: Vector2,
    pub velocity: Vector2,
    pub radius: f32,
    pub max_speed: f32,
}

impl Paddle {
    /// Create a new paddle at a given starting position
    pub fn new(player_num: u8, start_position: Vector2) -> Self {
        let position = start_position;
        let velocity = Vector2 { x: 0.0, y: 0.0 };
        let radius = 30.0;
        let max_speed = PADDLE_MAX_SPEED;
        
        Self {
            player_num,
            position,
            velocity,
            radius,
            max_speed,
        }
    }

    /// Update paddle position based on its velocity and elapsed time dt
    pub fn update(&mut self, dt: f32) {
        self.position.x += self.velocity.x * dt;
        self.position.y += self.velocity.y * dt;
    }

    /// Set paddle velocity (typically from player input)
    pub fn set_velocity(&mut self, vx: f32, vy: f32) {
        self.velocity.x = vx;
        self.velocity.y = vy;
        self.clamp_velocity();
    }

    /// Clamp the paddle velocity to its maximum speed
    pub fn clamp_velocity(&mut self) {
        let speed = (self.velocity.x.powi(2) + self.velocity.y.powi(2)).sqrt();
        if speed > self.max_speed {
            let scale = self.max_speed / speed;
            self.velocity.x *= scale;
            self.velocity.y *= scale;
        }
    }

    /// Manually move paddle to a position (e.g., teleporting or snapping)
    pub fn set_position(&mut self, pos: Vector2) {
        self.position = pos;
    }
}
