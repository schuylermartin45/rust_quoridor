//!
//! File:           main.rs
//! Description:    Player representation
//!

/// Player pawn color
pub enum Color {
    Red,
    Blue,
    Green,
    Yellow,
}

/// Represents a player
pub struct Player {
    id: Color,
    wins: usize,
}

impl Player {
    pub fn new(id: Color) -> Self {
        let player = Player { id: id, wins: 0 };
        player
    }
}
