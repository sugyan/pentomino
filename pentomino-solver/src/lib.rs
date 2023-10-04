mod pieces;
mod shapes;
pub mod solvers;

pub use pieces::{Piece, NUM_PIECES};
pub use solvers::Solver;

pub type Bitboard = u64;
