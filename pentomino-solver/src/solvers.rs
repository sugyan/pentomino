mod simple;

use crate::Piece;
pub use simple::SimpleSolver;

pub trait Solver {
    fn new(rows: usize, cols: usize) -> Self;
    fn solve(&self, start: u64) -> Vec<Vec<u64>>;
    fn represent_solution(&self, solution: &[u64]) -> Option<Vec<Vec<Option<Piece>>>>;
}
