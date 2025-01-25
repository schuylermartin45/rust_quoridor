//!
//! File:           board.rs
//! Description:    Game board representation
//!

use crate::player::{Color, Direction, Player};
use std::collections::{HashMap, HashSet};
use std::fmt;

/// Board
const BOARD_SIZE: isize = 5;

/// A coordinate position in the game board (row,column)
#[derive(Hash, PartialEq, Eq)]
struct Point(isize, isize);

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1);
        Ok(())
    }
}

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
    pub fn new(row: isize, col: isize, contents: Option<Color>) -> Self {
        let mut bn = BoardNode {
            contents: contents,
            neighbors: HashSet::new(),
        };

        let upper_bound = BOARD_SIZE - 1;
        if row < upper_bound {
            bn.neighbors.insert(Point(row + 1, col));
        }
        if row > 0 {
            bn.neighbors.insert(Point(row - 1, col));
        }
        if col < upper_bound {
            bn.neighbors.insert(Point(row, col + 1));
        }
        if col > 0 {
            bn.neighbors.insert(Point(row, col - 1));
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
    // Road-robin player tracker
    player_order: Vec<Player>,
    // Ever-incrementing turn counter determines the current player
    turn_cntr: usize,
}

impl Board {
    /// Initializes a new game board
    pub fn new() -> Self {
        // Initialize the board
        let mut board = Board {
            spaces: HashMap::new(),
            wall_spaces: HashSet::new(),
            player_tbl: HashMap::new(),
            // TODO 4 player?
            player_order: vec![Player::new(Color::Blue), Player::new(Color::Red)],
            // TODO alternate starting player between games
            turn_cntr: 0,
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

    /// Calculates the current player for which an operation should apply to.
    /// TODO: use Player struct, not color
    fn get_cur_player(&self) -> &Player {
        &self.player_order[self.turn_cntr % self.player_order.len()]
    }

    /// Indicates to the game state control that the next turn is taking place
    /// so the correct player is manipulated.
    fn next_turn(&mut self) {
        self.turn_cntr += 1;
    }

    /// Given a pawn, returns the list of possible directions the pawn may move.
    pub fn get_available_pawn_directions(&self) -> HashSet<Direction> {
        let color = self.get_cur_player().get_id();
        let player_pt = self.player_tbl.get(&color).unwrap();
        let neighbors = match self.spaces.get(&player_pt) {
            Some(bn) => &bn.neighbors,
            None => panic!("Unable to fetch neighbors for {}", player_pt),
        };

        let mut directions = HashSet::<Direction>::new();
        // There are at most 5 directions a pawn can move (when diagonals come
        // into play there is a case in which two directions replace one).
        directions.reserve(5);

        if neighbors.contains(&Point(player_pt.0 - 1, player_pt.1)) {
            directions.insert(Direction::N);
        }
        if neighbors.contains(&Point(player_pt.0, player_pt.1 + 1)) {
            directions.insert(Direction::E);
        }
        if neighbors.contains(&Point(player_pt.0 + 1, player_pt.1)) {
            directions.insert(Direction::S);
        }
        if neighbors.contains(&Point(player_pt.0, player_pt.1 - 1)) {
            directions.insert(Direction::W);
        }
        // TODO handle pawn jumping
        // TODO handle diagonal edge cases caused by wall/pawn blocking.
        // TODO consider panicking if no moves are available. This should never
        // happen if the game is played correctly.

        directions
    }

    /// Moves a pawn in a specified direction
    pub fn move_pawn(&mut self, direction: Direction) {
        let color = self.get_cur_player().get_id();
        let mut player_pt = self.player_tbl.get_mut(&color).unwrap();

        let new_pt = match direction {
            Direction::N => Point(player_pt.0 - 1, player_pt.1),
            Direction::E => Point(player_pt.0, player_pt.1 + 1),
            Direction::S => Point(player_pt.0 + 1, player_pt.1),
            Direction::W => Point(player_pt.0, player_pt.1 - 1),
            Direction::NE => Point(player_pt.0 - 1, player_pt.1 + 1),
            Direction::SE => Point(player_pt.0 + 1, player_pt.1 + 1),
            Direction::SW => Point(player_pt.0 + 1, player_pt.1 - 1),
            Direction::NW => Point(player_pt.0 - 1, player_pt.1 - 1),
        };

        if let Some(mut old_board_node) = self.spaces.get_mut(&player_pt) {
            old_board_node.contents = None;
        }
        if let Some(mut new_board_node) = self.spaces.get_mut(&new_pt) {
            new_board_node.contents = Some(color);
        }

        *player_pt = new_pt;

        // This is all a player can do this turn
        self.next_turn();
    }

    /// Indicates if a provided wall position is valid
    pub fn can_place_wall(&self, wall: Wall) -> bool {
        // TODO complete
        // Check all wall positions, if at least one coordinate matches, a
        // wall already exists in that position.
        false
    }

    /// Places a wall in a specific location
    pub fn place_wall(&mut self, player: &mut Player, wall: Wall) {
        // TODO complete
        // TODO remove neighbors from affected graph nodes.

        // This is all a player can do this turn
        self.next_turn();
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
                write!(f, "  Player Turn: {}", self.get_cur_player().get_id()).expect("I/O Error");
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
