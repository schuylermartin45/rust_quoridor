//!
//! File:           main.rs
//! Description:    Player representation
//!

use std::fmt;

pub const WALL_COUNT_2_PLAYERS: usize = 10;
pub const WALL_COUNT_4_PLAYERS: usize = 5;

/// Player pawn color
#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub enum Color {
    Red,
    Blue,
    Green,
    Yellow,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Color::Red => "Red",
                Color::Blue => "Blue",
                Color::Green => "Green",
                Color::Yellow => "Yellow",
            }
        );

        Ok(())
    }
}

/// Player pawn direction
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
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

    /// Retrieve the player's identifying color
    pub fn get_id(&self) -> Color {
        self.id
    }
}
