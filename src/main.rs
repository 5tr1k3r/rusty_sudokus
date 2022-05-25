mod config;
mod puzzle;
mod sudoku_solver;
mod tech;

use crate::puzzle::Puzzle;
use crate::sudoku_solver::{batch_solve, batch_solve_everything, solve};
use std::env;

fn start_puzzle_solving_mode(puzzle_string: &str) {
    println!("Puzzle solving mode");
    let mut my_puzzle = Puzzle::from_string(puzzle_string);

    solve(&mut my_puzzle);

    dbg!(my_puzzle);
}

fn start_batch_solving_mode(filename: &str) {
    println!("Batch solving mode");
    match filename {
        "all" => batch_solve_everything(),
        _ => batch_solve(filename),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let optional_args = &args[1..];
    if optional_args.is_empty() {
        sudoku_solver::run_default();
        return;
    } else if optional_args.len() != 2 {
        panic!("Need exactly two arguments");
    }

    let mode = &optional_args[0];
    let object = &optional_args[1];

    match mode.as_str() {
        "-p" => start_puzzle_solving_mode(object),
        "-b" => start_batch_solving_mode(object),
        _ => panic!("Unexpected argument(s)"),
    }
}
