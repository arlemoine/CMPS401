use crate::models::airhockey::{paddle::Paddle, puck::Puck, table::Table};

#[derive(Debug, Clone)]
pub enum CollisionType {
    None,
    Wall, // Collision with a vertical or horizontal wall
    Paddle(u8), // Collided with paddle (player number)
    Goal(u8), // Collided with goal area (player number who scored)
}

pub fn broad_phase(puck: &Puck, table: &Table, paddles: &std::collections::HashMap<u8, Paddle>) -> bool {
    // Quick table boundary check
    if puck.position.x - puck.radius <= 0.0
        || puck.position.x + puck.radius >= table.width
        || puck.position.y - puck.radius <= 0.0
        || puck.position.y + puck.radius >= table.height
    {
        return true;
    }

    // Quick paddle proximity check
    for paddle in paddles.values() {
        let dx = puck.position.x - paddle.position.x;
        let dy = puck.position.y - paddle.position.y;
        let distance_sq = dx * dx + dy * dy;
        let radius_sum = puck.radius + paddle.radius;

        if distance_sq <= radius_sum * radius_sum {
            return true;
        }
    }

    false
}

/// Narrow-phase collision detection
/// Precise collisions and response identification
pub fn narrow_phase(
    puck: &Puck,
    table: &Table,
) -> CollisionType {
    // Wall collision
    if puck.position.x - puck.radius <= 0.0 || puck.position.x + puck.radius >= table.width {
        return CollisionType::Wall;
    }

    if puck.position.y - puck.radius <= 0.0 || puck.position.y + puck.radius >= table.height {
        return CollisionType::Wall;
    }

    // Goal detection
    if puck.position.y - puck.radius <= 0.0 {
        // Top goal → player 2 scores
        return CollisionType::Goal(2);
    }

    if puck.position.y + puck.radius >= table.height {
        // Bottom goal → player 1 scores
        return CollisionType::Goal(1);
    }

    // Paddle collisions
    for (player_num, paddle) in table.paddles.iter() {
        let dx = puck.position.x - paddle.position.x;
        let dy = puck.position.y - paddle.position.y;
        let distance_sq = dx*dx + dy*dy;
        let radius_sum = puck.radius + paddle.radius;

        if distance_sq <= radius_sum * radius_sum {
            return CollisionType::Paddle(*player_num);
        }
    }

    CollisionType::None
}

/// Reflect puck velocity for a simple elastic collision with walls
pub fn reflect_wall(puck: &mut Puck, table_width: f32, table_height: f32) {
    if puck.position.x - puck.radius <= 0.0 || puck.position.x + puck.radius >= table_width {
        puck.velocity.x = -puck.velocity.x;
    }
    if puck.position.y - puck.radius <= 0.0 || puck.position.y + puck.radius >= table_height {
        puck.velocity.y = -puck.velocity.y;
    }
}

/// Reflect puck off a paddle using simple elastic collision
pub fn reflect_paddle(puck: &mut Puck, paddle: &Paddle) {
    let dx = puck.position.x - paddle.position.x;
    let dy = puck.position.y - paddle.position.y;
    let dist_sq = dx*dx + dy*dy;

    if dist_sq == 0.0 {
        // Avoid division by zero
        return;
    }

    let dist = dist_sq.sqrt();
    let nx = dx / dist;
    let ny = dy / dist;

    // Simple velocity reflection along normal
    let dot = puck.velocity.x * nx + puck.velocity.y * ny;
    puck.velocity.x = puck.velocity.x - 2.0 * dot * nx;
    puck.velocity.y = puck.velocity.y - 2.0 * dot * ny;
}
