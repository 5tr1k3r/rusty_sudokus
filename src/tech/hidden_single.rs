use crate::puzzle::{Puzzle, get_all_group_indices};
use crate::tech::base_tech::Technique;

pub struct HiddenSingle;

impl Technique for HiddenSingle {
    fn get_name(&self) -> &str {
        "HiddenSingle"
    }

    fn apply(&self, puzzle: &mut Puzzle) -> bool {
        let mut is_progress: bool = false;

        for group in get_all_group_indices() {
            let counter = puzzle.get_candidates_counter(&group);

            for (value, count) in counter {
                if count == 1 {
                    for (x, y) in puzzle.get_candidates_indices_by_value(value, &group) {
                        puzzle.assign_value_to_cell(value, x, y);
                        is_progress = true;
                    }
                }
            }
        }

        is_progress
    }
}

impl HiddenSingle {}
