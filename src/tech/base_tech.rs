use crate::puzzle::Puzzle;

pub trait Technique {
    fn apply(&self, puzzle: &mut Puzzle) -> bool;
}