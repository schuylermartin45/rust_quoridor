//!
//! File:           main.rs
//! Description:    Player representation
//!

use rstest::rstest;
use std::fmt;

pub const WALL_COUNT_2_PLAYERS: isize = 10;
pub const WALL_COUNT_4_PLAYERS: isize = 5;

/// Player pawn color
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
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
                Color::Red => "\x1b[91mRed\x1b[0m",
                Color::Blue => "\x1b[94mBlue\x1b[0m",
                Color::Green => "\x1b[92mGreen\x1b[0m",
                Color::Yellow => "\x1b[93mYellow\x1b[0m",
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
    pub fn get_wall_count(&self) -> isize {
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

/**** Unit tests for Player functions ****/

/// Check that when a wall is used, the wall count state is managed correctly.
#[rstest]
#[case(Player::new(Color::Red), Color::Red)]
#[case(Player::new(Color::Blue), Color::Blue)]
#[case(Player::new(Color::Green), Color::Green)]
#[case(Player::new(Color::Yellow), Color::Yellow)]
fn validate_get_id(#[case] player: Player, #[case] expected: Color) {
    assert_eq!(player.get_id(), expected)
}

/// Check that when a player wins, the win counter state is managed correctly.
#[rstest]
#[case(Player::new(Color::Blue), 0)]
#[case(Player::new(Color::Blue), 6)]
#[case(Player::new(Color::Blue), 5)]
#[case(Player::new(Color::Blue), 42)]
#[case(Player::new(Color::Blue), 343)]
fn validate_win_tracking(#[case] mut player: Player, #[case] expected: usize) {
    for _ in 0..expected {
        player.player_won();
    }
    assert_eq!(player.get_win_count(), expected)
}

/// Check that when a wall is used, the wall count state is managed correctly.
#[rstest]
#[case(Player::new(Color::Blue), 0, 10)]
#[case(Player::new(Color::Blue), 6, 4)]
#[case(Player::new(Color::Blue), 5, 5)]
#[case(Player::new(Color::Blue), 10, 0)]
#[case(Player::new(Color::Blue), 11, 0)]
fn validate_wall_usage(
    #[case] mut player: Player,
    #[case] walls_used: isize,
    #[case] expected: isize,
) {
    for _ in 0..walls_used {
        player.use_wall();
    }
    assert_eq!(player.get_wall_count(), expected)
}
