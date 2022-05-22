use crate::puzzle::Puzzle;
use crate::tech::single_candidate::SingleCandidate;
use crate::tech::base_tech::ApplyTechnique;


fn solve(puzzle: &mut Puzzle){
    while !puzzle.check_if_solved() {
        let progress = SingleCandidate.apply(puzzle);
        if progress == false {
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
