//!
//! File:           main.rs
//! Description:    Player representation
//!

use std::fmt;

pub const WALL_COUNT_2_PLAYERS: isize = 10;
pub const WALL_COUNT_4_PLAYERS: isize = 5;

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
    wall_cntr: isize,
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

    /// Retrieve the number of times a player has won
    pub fn get_win_count(&self) -> usize {
        self.wins
    }

    /// Tracks a player's win
    pub fn player_won(&mut self) {
        self.wins += 1;
    }

    /// Retrieve the number of walls a player has left at their disposal
    pub fn get_wall_count(self) -> isize {
        self.wall_cntr
    }

    /// Tracks wall usage
    pub fn use_wall(&mut self) {
        // Forbid a negative wall count
        if self.wall_cntr == 0 {
            return;
        }
        self.wall_cntr -= 1;
    }
}
