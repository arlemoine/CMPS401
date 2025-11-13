/// Game screen dimensions (pixels)
pub const WIDTH: f32 = 1280.0; // typical landscape resolution
pub const HEIGHT: f32 = 720.0; // 16:9 aspect ratio

/// Paddle configuration
pub const PADDLE_MAX_SPEED: f32 = 600.0; // pixels per second

/// Puck configuration
pub const PUCK_MAX_SPEED: f32 = 800.0; // pixels per second
pub const PUCK_FRICTION: f32 = 0.995; // velocity multiplier per tick

/// Net/goal configuration
pub const NET_WIDTH_RATIO: f32 = 0.2; // 20% of table width
pub const GOAL_MARGIN: f32 = 10.0; // distance from top/bottom edges

/// Physics / collisions
pub const WALL_BOUNCE_DAMPING: f32 = 0.9; // velocity multiplier when puck hits wall
pub const PADDLE_BOUNCE_DAMPING: f32 = 1.0; // can tweak for elastic collisions

/// Timing
pub const FIXED_DT: f32 = 1.0 / 60.0; // 60 ticks per second

/// Score configuration
pub const WINNING_SCORE: f32 = 7.0; // first player to reach this score wins
