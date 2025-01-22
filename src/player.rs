//!
//! File:           main.rs
//! Description:    Player representation
//!

pub const WALL_COUNT_2_PLAYERS: usize = 10;
pub const WALL_COUNT_4_PLAYERS: usize = 5;

/// Player pawn color
pub enum Color {
    Red,
    Blue,
    Green,
    Yellow,
}

/// Player pawn direction
pub enum Direction {
    N,
    E,
    S,
    W,
    // Only applicable in special passing edge cases.
    NE,
    NW,
    SE,
    SW,
}

/// Represents a player
pub struct Player {
    id: Color,
    wins: usize,
    wall_cntr: usize,
}

impl Player {
    pub fn new(id: Color) -> Self {
        let player = Player {
            id: id,
            wins: 0,
            // TODO: 4 player support?
            wall_cntr: WALL_COUNT_2_PLAYERS,
        };
        player
    }
}
