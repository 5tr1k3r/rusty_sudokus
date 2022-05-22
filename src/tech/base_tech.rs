use crate::puzzle::Puzzle;

pub trait ApplyTechnique {
    fn apply(&self, puzzle: &mut Puzzle) -> bool;
}