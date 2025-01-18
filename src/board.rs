//!
//! File:           board.rs
//! Description:    Game board representation
//!

use crate::player::Color;
use std::collections::{HashMap, HashSet};
use std::fmt;

/// Board
const BOARD_SIZE: usize = 5;

/// A coordinate position in the game board (row,column)
#[derive(Hash, PartialEq, Eq)]
struct Point {
    row: usize,
    col: usize,
}

impl Point {
    /// Convenience constructor for the Point type
    pub fn new(r: usize, c: usize) -> Self {
        let p = Point { row: r, col: c };
        p
    }
}

// TODO Walls defined as 2 upper left points?

/// Represents a single space on the board
struct BoardNode {
    contents: Option<Color>,
    neighbors: HashSet<Point>,
}

impl BoardNode {
    /// Constructs a BoardNode instance, automatically calculating initial
    /// neighbor positions
    pub fn new(row: usize, col: usize, contents: Option<Color>) -> Self {
        let mut bn = BoardNode {
            contents: contents,
            neighbors: HashSet::new(),
        };

        let upper_bound = BOARD_SIZE - 1;
        if row < upper_bound && col < upper_bound {
            bn.neighbors.insert(Point::new(row + 1, col + 1));
        }
        if row < upper_bound && col > 0 {
            bn.neighbors.insert(Point::new(row + 1, col - 1));
        }
        if row > 0 && col < upper_bound {
            bn.neighbors.insert(Point::new(row - 1, col + 1));
        }
        if row > 0 && col > 0 {
            bn.neighbors.insert(Point::new(row - 1, col - 1));
        }

        bn
    }
}

/// Represents the board state
pub struct Board {
    spaces: HashMap<Point, BoardNode>,
}

impl Board {
    /// Initializes a new game board
    pub fn new() -> Self {
        // Initialize the board
        let mut board = Board {
            spaces: HashMap::new(),
        };
        for r in 0..BOARD_SIZE {
            for c in 0..BOARD_SIZE {
                board
                    .spaces
                    .insert(Point::new(r, c), BoardNode::new(r, c, None));
            }
        }

        // Place player colors
        // TODO 4 player?
        if let Some(bn) = board.spaces.get_mut(&Point::new(0, 2)) {
            (*bn).contents = Some(Color::Red);
        }
        if let Some(bn) = board.spaces.get_mut(&Point::new(4, 2)) {
            (*bn).contents = Some(Color::Blue);
        }

        board
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for r in 0..BOARD_SIZE {
            for c in 0..BOARD_SIZE {
                // TODO print walls

                // Determine player character to render on the board
                let pos_ch = match self.spaces.get(&Point::new(r, c)) {
                    Some(bn) => match bn.contents {
                        Some(Color::Red) => "R",
                        Some(Color::Blue) => "B",
                        Some(Color::Green) => "G",
                        Some(Color::Yellow) => "Y",
                        None => "_",
                    },
                    None => "_",
                };
                write!(f, "  [{}]", pos_ch).expect("I/O Error");
            }
            if r == 0 {
                write!(f, "  Player Turn: TODO").expect("I/O Error");
            }
            writeln!(f, "").expect("I/O Error");
            if r == 0 {
                write!(f, "{:>43} {:02}", "Walls remaining:", 0).expect("I/O Error");
            }
            writeln!(f, "").expect("I/O Error");
        }

        Ok(())
    }
}
