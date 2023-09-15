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
    fn backtrack(
        &self,
        current: Bitboard,
        remain: u32,
        limit: Option<usize>,
        pieces: &mut [Bitboard; NUM_PIECES],
        solutions: &mut Vec<[Bitboard; NUM_PIECES]>,
    ) -> bool {
        if remain == 0 {
            solutions.push(*pieces);
            return limit.map_or(false, |l| solutions.len() >= l);
        }
        let target = u64::from(current).trailing_ones() as usize;
        for (i, candidate) in self.table[target].iter().enumerate() {
            if remain & (1 << i) != 0 {
                for &b in candidate.iter() {
                    if (current & b).is_empty() {
                        pieces[i] = b;
                        if self.backtrack(current | b, remain & !(1 << i), limit, pieces, solutions)
                        {
                            return true;
                        }
                        pieces[i] = Bitboard::from(0);
                    }
                }
            }
        }
        false
    }
}

impl Solver for SimpleSolver {
    fn new(rows: usize, cols: usize) -> Self {
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
    fn solve(&self, initial: Bitboard, limit: Option<usize>) -> Vec<[Bitboard; NUM_PIECES]> {
        let mut solutions = Vec::new();
        self.backtrack(
            initial,
            (1 << NUM_PIECES) - 1,
            limit,
            &mut [Bitboard::from(0); NUM_PIECES],
            &mut solutions,
        );
        solutions
    }
    fn represent_solution(&self, solution: &[Bitboard]) -> Vec<Vec<Option<Piece>>> {
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
