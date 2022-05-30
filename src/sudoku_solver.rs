use crate::config::SOLVE_OUTPUT_ENABLED;
use crate::puzzle::Puzzle;
use crate::tech::base_tech::Technique;
use crate::tech::hidden_single::HiddenSingle;
use crate::tech::single_candidate::SingleCandidate;
use rayon::prelude::*;
use std::fs;
use std::time::Instant;

pub fn solve(original_puzzle: Puzzle) -> bool {
    let mut queue: Vec<Puzzle> = vec![original_puzzle];

    while !queue.is_empty() {
        let mut puzzle = queue.pop().unwrap();

        if solve_logically(&mut puzzle) {
            notify_puzzle_solved();
            return true
        }

        if puzzle.is_impossible() {
            notify_puzzle_impossible();
            continue;
        }

        let (x, y) = puzzle.find_cell_with_fewest_candidates();
        notify_picking_cell_to_bruteforce(x, y);

        for cand in puzzle.candidates[y][x].ones() {
            let mut new_puzzle: Puzzle = puzzle.copy();
            new_puzzle.assign_value_to_cell(cand, x, y);
            queue.push(new_puzzle);
        }
    }

    false
}

pub fn solve_logically(puzzle: &mut Puzzle) -> bool {
    let techs: Vec<Box<dyn Technique>> = vec![
        Box::new(SingleCandidate) as Box<dyn Technique>,
        Box::new(HiddenSingle) as _,
    ];
    let mut is_validated = false;

    while !puzzle.is_solved() {
        let mut progress = false;
        for tech in &techs {
            notify_applying_technique(tech);
            progress = tech.apply(puzzle) || progress;
        }

        if !progress {
            notify_no_progress();
            break;
        }
    }

    if puzzle.is_solved() {
        is_validated = puzzle.validate_solution();
        if !is_validated {
            notify_solution_invalid();
        }
    }

    is_validated
}

fn solve_puzzle_by_string(puzzle_string: &str) -> bool {
    let puzzle = Puzzle::from_string(puzzle_string);
    solve(puzzle)
}

pub fn batch_solve(filename: &str) {
    let all_puzzles = fs::read_to_string(filename).expect("File reading error");

    let time_start = Instant::now();

    let total_count: usize = all_puzzles.lines().count();
    let unsolved_count: usize = all_puzzles
        .par_lines()
        .map(solve_puzzle_by_string)
        .filter(|x| !x)
        .count();

    let time_taken = time_start.elapsed().as_secs_f64();

    let result_string = construct_result_string(filename, total_count, unsolved_count, time_taken);
    println!("{}", result_string);
}

pub fn batch_solve_everything() {
    let files = [
        "batches/0.txt",
        "batches/1.txt",
        "batches/2.txt",
        "batches/3.txt",
        "batches/5.txt",
    ];
    for file in files {
        batch_solve(file);
    }
}

fn construct_result_string(
    filename: &str,
    total_count: usize,
    unsolved_count: usize,
    time_taken: f64,
) -> String {
    let mut output: Vec<String> = vec![format!("{}", filename)];

    let unsolved_rate: f32 = unsolved_count as f32 / total_count as f32;

    output.push(format!(
        "Total: {}, unsolved: {} ({:.1}%), took {}s",
        total_count,
        unsolved_count,
        unsolved_rate * 100.0,
        time_taken
    ));

    output.join("\n")
}

fn notify_applying_technique(tech: &Box<dyn Technique>) {
    if SOLVE_OUTPUT_ENABLED {
        println!("Applying {} technique", tech.get_name());
    }
}

fn notify_no_progress() {
    if SOLVE_OUTPUT_ENABLED {
        println!("No progress detected, stopping the solve");
    }
}

fn notify_solution_invalid() {
    println!("Solution is invalid!");
}

fn notify_puzzle_impossible() {
    if SOLVE_OUTPUT_ENABLED {
        println!("Puzzle is impossible to solve!");
    }
}

fn notify_puzzle_solved() {
    if SOLVE_OUTPUT_ENABLED {
        println!("Puzzle solved");
    }
}

fn notify_picking_cell_to_bruteforce(x: usize, y: usize) {
    if SOLVE_OUTPUT_ENABLED {
        println!("Going to pick cell {}, {} and bruteforce from there", x, y);
    }
}

pub fn run_default() {
    // let pstring: &str =
    //     "030072001000030090518000003050203100000705306000640205200060014007000630000008900";
    // let mut my_puzzle = Puzzle::from_string(pstring);

    // solve(my_puzzle);

    // batch_solve("batches/5.txt");
    // batch_solve_everything();

    println!("Available commands:");
    println!("  -p <puzzle_string>     Solve a puzzle. Input: puzzle string.");
    println!("  Example: -p 030072001000030090518000003050203100000705306000640205200060014007000630000008900\n");
    println!("  -b <filename>          Solve a batch with puzzles. Input: filename.");
    println!("  Example: -b batches/0.txt\n");
    println!("  -b all                 Solve all batches.");
}
