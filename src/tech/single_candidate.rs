use crate::puzzle::{Puzzle, SIZE};
use crate::tech::base_tech::ApplyTechnique;

pub struct SingleCandidate;

impl ApplyTechnique for SingleCandidate {
    fn apply(&self, puzzle: &mut Puzzle) -> bool {
        println!("Applying SingleCandidate technique");

        let mut is_progress: bool = false;

        for y in 0..SIZE {
            for x in 0..SIZE {
                let cands = puzzle.candidates[y][x].clone();
                if cands.len() == 1 {
                    let value = cands.iter().next().unwrap().clone();
                    puzzle.assign_value_to_cell(value, x, y);
                    is_progress = true;
                }
            }
        }

        is_progress
    }
}

impl SingleCandidate {}