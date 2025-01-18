//!
//! File:           main.rs
//! Description:    CLI interface for this project
//!

pub mod board;
pub mod player;

use self::board::Board;
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

/// Initialize a single game instance
fn run_game() {
    let board = Board::new();

    clr_scr();

    loop {
        // Render board state
        println!("{}", board);

        // Render user prompt
        let input = prompt_read_str("(M)ove | (Q)uit > ");
        match input.as_str() {
            "e" | "exit" | "q" | "quit" => {
                process::exit(0);
            }
            _ => (),
        }
    }
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
