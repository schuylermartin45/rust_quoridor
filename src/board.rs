//!
//! File:           board.rs
//! Description:    Game board representation
//!

use crate::player::{Color, Direction, Player};
use std::collections::{HashMap, HashSet};
use std::fmt;

/// Board
const BOARD_SIZE: usize = 5;

/// A coordinate position in the game board (row,column)
#[derive(Hash, PartialEq, Eq)]
struct Point(usize, usize);

/// A wall is just two pairs of coordinates. Uses the same row/column coordinate
/// system as the rest of the board (row/column refers to the upper and left
/// side of a board position)
#[derive(Hash, PartialEq, Eq)]
struct Wall(Point, Point);

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
            bn.neighbors.insert(Point(row + 1, col + 1));
        }
        if row < upper_bound && col > 0 {
            bn.neighbors.insert(Point(row + 1, col - 1));
        }
        if row > 0 && col < upper_bound {
            bn.neighbors.insert(Point(row - 1, col + 1));
        }
        if row > 0 && col > 0 {
            bn.neighbors.insert(Point(row - 1, col - 1));
        }

        bn
    }
}

/// Represents the board state
pub struct Board {
    // Board positions are stored as a graph of neighbors
    spaces: HashMap<Point, BoardNode>,
    // Walls are stored as points that are current occupied by a wall.
    wall_spaces: HashSet<Point>,
    // Look-up table of all player pieces
    player_tbl: HashMap<Color, Point>,
}

impl Board {
    /// Initializes a new game board
    pub fn new() -> Self {
        // Initialize the board
        let mut board = Board {
            spaces: HashMap::new(),
            wall_spaces: HashSet::new(),
            player_tbl: HashMap::new(),
        };
        for r in 0..BOARD_SIZE {
            for c in 0..BOARD_SIZE {
                board.spaces.insert(Point(r, c), BoardNode::new(r, c, None));
            }
        }

        // Place player colors
        // TODO 4 player?
        let player_start_top = Point(0, 2);
        let player_start_bottom = Point(4, 2);

        if let Some(bn) = board.spaces.get_mut(&player_start_top) {
            (*bn).contents = Some(Color::Red);
        }
        board.player_tbl.insert(Color::Red, player_start_top);
        if let Some(bn) = board.spaces.get_mut(&player_start_bottom) {
            (*bn).contents = Some(Color::Blue);
        }
        board.player_tbl.insert(Color::Blue, player_start_bottom);

        board
    }

    /// Given a pawn, returns the list of possible directions the pawn may move.
    pub fn available_pawn_directions(self, color: Color) -> Vec<Direction> {
        let player_pt = match self.player_tbl.get(&color) {
            Some(p) => p,
            None => return Vec::new(),
        };
        let neighbors = match self.spaces.get(&player_pt) {
            Some(bn) => &bn.neighbors,
            None => return Vec::new(),
        };

        let mut directions = Vec::<Direction>::new();
        // There are at most 5 directions a pawn can move (when diagonals come
        // into play there is a case in which two directions replace one).
        directions.reserve(5);

        // Note that subtraction on an unsigned 0 should be safe as we checked
        // the bounds on neighbor initialization. Large array indices will not
        // be found in a 5x5 playing board.
        if neighbors.contains(&Point(player_pt.0 - 1, player_pt.1)) {
            directions.push(Direction::N);
        }
        if neighbors.contains(&Point(player_pt.0, player_pt.1 + 1)) {
            directions.push(Direction::E);
        }
        if neighbors.contains(&Point(player_pt.0 + 1, player_pt.1)) {
            directions.push(Direction::S);
        }
        if neighbors.contains(&Point(player_pt.0, player_pt.1 - 1)) {
            directions.push(Direction::W);
        }

        directions
    }

    /// Moves a pawn in a specified direction
    pub fn move_pawn(&mut self, color: Color, direction: Direction) {}

    /// Indicates if a provided wall position is valid
    pub fn can_place_wall(self, wall: Wall) -> bool {
        // TODO complete
        // Check all wall positions, if at least one coordinate matches, a
        // wall already exists in that position.
        false
    }

    /// Places a wall in a specific location
    pub fn place_wall(&mut self, player: &mut Player, wall: Wall) {
        // TODO complete
        // TODO remove neighbors from affected graph nodes.
    }
}

/*
Board Display Diagram
---------------------

    0    1    2    3    4

0  [_]  [_]  [R]  [_]  [_]  Player Turn: TODO
                            Walls remaining: 00
1  [_]||[_]  [_]  [_]  [_]
      ||
2  [_]||[_]  [_]  [_]  [_]
        ========
3  [_]  [_]  [_]  [_]  [_]

4  [_]  [_]  [B]  [_]  [_]

*/
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for r in 0..BOARD_SIZE {
            for c in 0..BOARD_SIZE {
                // TODO print walls

                // Determine player character to render on the board
                let pos_ch = match self.spaces.get(&Point(r, c)) {
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
