use crate::puzzle::Puzzle;

pub trait Technique {
    fn get_name(&self) -> &str;
    fn apply(&self, puzzle: &mut Puzzle) -> bool;
}
