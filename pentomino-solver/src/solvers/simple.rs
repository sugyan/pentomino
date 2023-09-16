use super::Solver;
use crate::shapes::calculate_shapes;
use crate::{Bitboard, Piece, NUM_PIECES};
use num_traits::FromPrimitive;
use std::array;

pub struct SimpleSolver {
    rows: usize,
    cols: usize,
    table: [[Vec<Bitboard>; NUM_PIECES]; 64],
}

impl SimpleSolver {
    pub fn new(rows: usize, cols: usize) -> Self {
        assert!(rows * cols <= 64);
        let shapes = calculate_shapes();
        let mut table = array::from_fn(|_| array::from_fn(|_| Vec::new()));
        for (n, shape) in shapes.iter().enumerate() {
            for s in shape {
                if s.iter().any(|&(x, y)| x >= cols || y >= rows) {
                    continue;
                }
                let v = s.iter().map(|p| (1 << (p.0 + p.1 * cols))).sum::<u64>();
                let (w, h) = s
                    .iter()
                    .fold((0, 0), |(xmax, ymax), &(x, y)| (xmax.max(x), ymax.max(y)));
                for i in 0..cols - w {
                    for j in 0..rows - h {
                        let offset = i + j * cols;
                        table[s[0].0 + offset][n].push((v << offset).into());
                    }
                }
            }
        }
        Self { rows, cols, table }
    }
    fn backtrack(
        &self,
        current: Bitboard,
        solutions: &mut Vec<[Bitboard; NUM_PIECES]>,
        s: &mut [Bitboard; NUM_PIECES],
    ) {
        if !s.iter().any(Bitboard::is_empty) {
            return solutions.push(*s);
        }
        let target = u64::from(current).trailing_ones() as usize;
        for i in 0..NUM_PIECES {
            if s[i].is_empty() {
                for &b in self.table[target][i].iter() {
                    if (current & b).is_empty() {
                        s[i] = b;
                        self.backtrack(current | b, solutions, s);
                        s[i] = Bitboard::default();
                    }
                }
            }
        }
    }
}

impl Solver for SimpleSolver {
    fn solve(&self, initial: Bitboard, unique: bool) -> Vec<[Bitboard; NUM_PIECES]> {
        if unique {
            panic!("SimpleSolver does not support unique solutions");
        }
        let mut solutions = Vec::new();
        self.backtrack(
            initial,
            &mut solutions,
            &mut [Bitboard::default(); NUM_PIECES],
        );
        solutions
    }
    fn represent_solution(&self, solution: &[Bitboard; NUM_PIECES]) -> Vec<Vec<Option<Piece>>> {
        let mut ret = vec![vec![None; self.cols]; self.rows];
        for (i, b) in solution.iter().enumerate() {
            let p = Piece::from_usize(i);
            for (y, row) in ret.iter_mut().enumerate() {
                for (x, col) in row.iter_mut().enumerate() {
                    if !(*b & Bitboard::from(1 << (x + y * self.cols))).is_empty() {
                        *col = p;
                    }
                }
            }
        }
        ret
    }
}
