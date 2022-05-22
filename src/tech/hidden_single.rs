use crate::puzzle::Puzzle;
use crate::tech::base_tech::Technique;

pub struct HiddenSingle;

impl Technique for HiddenSingle {
    fn apply(&self, puzzle: &mut Puzzle) -> bool {
        println!("Applying HiddenSingle technique");

        let mut is_progress: bool = false;

        for group in Puzzle::get_all_group_indices() {
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