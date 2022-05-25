use crate::puzzle::{Puzzle, SIZE};
use crate::tech::base_tech::Technique;

pub struct SingleCandidate;

impl Technique for SingleCandidate {
    fn get_name(&self) -> &str {
        "SingleCandidate"
    }

    fn apply(&self, puzzle: &mut Puzzle) -> bool {
        let mut is_progress: bool = false;

        for y in 0..SIZE {
            for x in 0..SIZE {
                let cands = puzzle.candidates[y][x].clone();
                if cands.count_ones(..) == 1 {
                    let value = cands.ones().next().unwrap();
                    puzzle.assign_value_to_cell(value, x, y);
                    is_progress = true;
                }
            }
        }

        is_progress
    }
}

impl SingleCandidate {}
