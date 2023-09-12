pub mod solvers;

use std::fmt::Write;

use num_derive::FromPrimitive;

pub const NUM_PIECES: usize = 12;

#[derive(Debug, Copy, Clone, PartialEq, Eq, FromPrimitive)]
pub enum Piece {
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Self::O => 'O',
            Self::P => 'P',
            Self::Q => 'Q',
            Self::R => 'R',
            Self::S => 'S',
            Self::T => 'T',
            Self::U => 'U',
            Self::V => 'V',
            Self::W => 'W',
            Self::X => 'X',
            Self::Y => 'Y',
            Self::Z => 'Z',
        })
    }
}

pub trait Solver {
    fn new(rows: usize, cols: usize) -> Self;
    fn solve(&self, start: u64) -> Vec<Vec<u64>>;
    fn represent_solution(&self, solution: &[u64]) -> Option<Vec<Vec<Option<Piece>>>>;
}
