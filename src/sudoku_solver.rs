use crate::puzzle::Puzzle;
use crate::tech::single_candidate::SingleCandidate;
use crate::tech::hidden_single::HiddenSingle;
use crate::tech::base_tech::Technique;


fn solve(puzzle: &mut Puzzle){
    let techs: Vec<Box<dyn Technique>> = vec![
        Box::new(SingleCandidate) as Box<dyn Technique>,
        Box::new(HiddenSingle) as _,
    ];

    while !puzzle.check_if_solved() {
        let mut progress = false;
        for tech in &techs {
            progress = tech.apply(puzzle) || progress;
        }

        if progress == false {
            println!("No progress detected, stopping the solve");
            break;
        }
    }
}

pub fn run() {
    let pstring: String = "030072001000030090518000003050203100000705306000640205200060014007000630000008900".to_string();
    let mut my_puzzle = Puzzle::from_string(pstring);

    solve(&mut my_puzzle);

    dbg!(my_puzzle);
}
