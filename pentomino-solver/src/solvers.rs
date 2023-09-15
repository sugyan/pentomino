mod simple;

use crate::{Bitboard, Piece, NUM_PIECES};
pub use simple::SimpleSolver;

pub trait Solver {
    fn new(rows: usize, cols: usize) -> Self;
    fn solve(
        &self,
        initial: Bitboard,
        unique: bool,
        limit: Option<usize>,
    ) -> Vec<[Bitboard; NUM_PIECES]>;
    fn represent_solution(&self, solution: &[Bitboard]) -> Vec<Vec<Option<Piece>>>;
}
