mod default;
mod optimized;
mod simple;

use crate::{Bitboard, Piece, NUM_PIECES};
pub use default::DefaultSolver;
pub use optimized::OptimizedSolver;
pub use simple::SimpleSolver;

pub trait Solver {
    fn solve(&self, initial: Bitboard, unique: bool) -> Vec<[Bitboard; NUM_PIECES]>;
    fn represent_solution(&self, solution: &[Bitboard; NUM_PIECES]) -> Vec<Vec<Option<Piece>>>;
}
