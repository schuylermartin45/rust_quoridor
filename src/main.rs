//!
//! File:           main.rs
//! Description:    CLI interface for this project
//!

pub mod board;
pub mod player;

use crate::board::Board;
use crate::player::Direction;
use std::io::{self, Write};
use std::process;

/// Clear screen function
fn clr_scr() {
    print!("\x1B[2J");
}

/// Convenience function that prompts the user for input and returns a
/// sanitized string.
fn prompt_read_str(prompt: &str) -> String {
    print!("{}", prompt);
    let _ = io::stdout().flush();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read user input");

    input.trim().to_lowercase()
}

/// Movement menu
fn move_menu(board: &mut Board) {
    loop {
        clr_scr();
        println!("{}", board);

        let pawn_dirs = board.get_available_pawn_directions();

        // Although verbose, this menu system guarantees moves are always listed
        // in the same order.
        let mut dir_prompt = String::from("Direction (");
        if pawn_dirs.contains(&Direction::N) {
            dir_prompt += "N ";
        }
        if pawn_dirs.contains(&Direction::NE) {
            dir_prompt += "NE ";
        }
        if pawn_dirs.contains(&Direction::E) {
            dir_prompt += "E ";
        }
        if pawn_dirs.contains(&Direction::SE) {
            dir_prompt += "SE ";
        }
        if pawn_dirs.contains(&Direction::S) {
            dir_prompt += "S ";
        }
        if pawn_dirs.contains(&Direction::SW) {
            dir_prompt += "SW ";
        }
        if pawn_dirs.contains(&Direction::W) {
            dir_prompt += "W ";
        }
        if pawn_dirs.contains(&Direction::NW) {
            dir_prompt += "NW ";
        }
        dir_prompt = String::from(dir_prompt.trim_end());
        dir_prompt += ") > ";

        // Render user prompt
        let input = prompt_read_str(&dir_prompt);
        match input.as_str() {
            // TODO conditional option
            // TODO rare directions
            "n" | "north" if pawn_dirs.contains(&Direction::N) => {
                board.move_pawn(Direction::N);
                break;
            }
            "e" | "east" if pawn_dirs.contains(&Direction::E) => {
                board.move_pawn(Direction::E);
                break;
            }
            "w" | "west" if pawn_dirs.contains(&Direction::W) => {
                board.move_pawn(Direction::W);
                break;
            }
            "s" | "south" if pawn_dirs.contains(&Direction::S) => {
                board.move_pawn(Direction::S);
                break;
            }
            // Note the overlap with `(E)ast` and `(E)xit` in this menu.
            "b" | "back" | "exit" => break,
            "q" | "quit" => {
                process::exit(0);
            }
            _ => (),
        }
    }
    clr_scr();
}

/// Initialize a single game instance
fn run_game() {
    let mut board = Board::new();

    loop {
        clr_scr();
        // Render board state
        println!("{}", board);

        // Render user prompt
        let input = prompt_read_str("(M)ove | (B)ack | (Q)uit > ");
        // TODO game over, exit game menu
        match input.as_str() {
            "m" | "move" => move_menu(&mut board),
            "b" | "back" | "e" | "exit" => break,
            "q" | "quit" => {
                process::exit(0);
            }
            _ => (),
        }
    }
    clr_scr();
}

/// Main menu for the game
fn main_menu() {
    loop {
        clr_scr();
        println!("==== Welcome to Quoridor! ====");
        println!("  [1]   Start a 2 player game (human vs human)");
        // TODO add computer opponent
        //println!("  [2] Start a 2 player game (human vs comp)");
        //println!("  [3] Start a 2 player game (comp vs comp)");
        println!("  [q|e] Quit/exit");

        let input = prompt_read_str("> ");
        // Quit the game from this sub-menu or set the old bet as the current.
        match input.as_str() {
            "1" => run_game(),
            "e" | "exit" | "q" | "quit" => {
                println!("Good-bye!");
                process::exit(0);
            }
            _ => (),
        }
    }
}

/// Executes primary game loop and CLI logic
fn main() {
    loop {
        clr_scr();
        main_menu();
    }
}
