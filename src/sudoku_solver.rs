use crate::config::SOLVE_OUTPUT_ENABLED;
use crate::puzzle::Puzzle;
use crate::tech::base_tech::Technique;
use crate::tech::hidden_single::HiddenSingle;
use crate::tech::single_candidate::SingleCandidate;

fn solve(puzzle: &mut Puzzle) -> bool {
    let techs: Vec<Box<dyn Technique>> = vec![
        Box::new(SingleCandidate) as Box<dyn Technique>,
        Box::new(HiddenSingle) as _,
    ];
    let mut is_validated = false;

    while !puzzle.check_if_solved() {
        let mut progress = false;
        for tech in &techs {
            notify_applying_technique(tech);
            progress = tech.apply(puzzle) || progress;
        }

        if progress == false {
            notify_no_progress();
            break;
        }
    }

    if puzzle.check_if_solved() {
        is_validated = puzzle.validate_solution();
        if !is_validated {
            notify_solution_invalid();
        }
    }

    is_validated
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

pub fn run() {
    let pstring: String =
        "030072001000030090518000003050203100000705306000640205200060014007000630000008900"
            .to_string();
    let mut my_puzzle = Puzzle::from_string(pstring);

    solve(&mut my_puzzle);

    dbg!(my_puzzle);
}
