mod simple;

use crate::{Bitboard, Piece};
pub use simple::SimpleSolver;

pub trait Solver {
    fn new(rows: usize, cols: usize) -> Self;
    fn solve(&self, initial: Bitboard) -> Vec<Vec<Bitboard>>;
    fn represent_solution(&self, solution: &[Bitboard]) -> Option<Vec<Vec<Option<Piece>>>>;
}
