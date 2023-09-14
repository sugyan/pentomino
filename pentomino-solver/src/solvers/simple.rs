use super::Solver;
use crate::{Bitboard, Piece, NUM_PIECES};
use num_traits::FromPrimitive;
use std::array;

pub struct SimpleSolver {
    rows: usize,
    cols: usize,
    table: [[Vec<Bitboard>; NUM_PIECES]; 64],
}

impl Solver for SimpleSolver {
    fn new(rows: usize, cols: usize) -> Self {
        assert!(rows * cols <= 64);

        let shapes = [
            vec![
                [(0, 0), (0, 1), (0, 2), (0, 3), (0, 4)],
                [(0, 0), (1, 0), (2, 0), (3, 0), (4, 0)],
            ],
            vec![
                [(0, 0), (1, 0), (0, 1), (1, 1), (0, 2)],
                [(0, 0), (1, 0), (2, 0), (1, 1), (2, 1)],
                [(1, 0), (0, 1), (1, 1), (0, 2), (1, 2)],
                [(0, 0), (1, 0), (0, 1), (1, 1), (2, 1)],
                [(0, 0), (1, 0), (0, 1), (1, 1), (1, 2)],
                [(1, 0), (2, 0), (0, 1), (1, 1), (2, 1)],
                [(0, 0), (0, 1), (1, 1), (0, 2), (1, 2)],
                [(0, 0), (1, 0), (2, 0), (0, 1), (1, 1)],
            ],
            vec![
                [(0, 0), (1, 0), (1, 1), (1, 2), (1, 3)],
                [(3, 0), (0, 1), (1, 1), (2, 1), (3, 1)],
                [(0, 0), (0, 1), (0, 2), (0, 3), (1, 3)],
                [(0, 0), (1, 0), (2, 0), (3, 0), (0, 1)],
                [(0, 0), (1, 0), (0, 1), (0, 2), (0, 3)],
                [(0, 0), (1, 0), (2, 0), (3, 0), (3, 1)],
                [(1, 0), (1, 1), (1, 2), (0, 3), (1, 3)],
                [(0, 0), (0, 1), (1, 1), (2, 1), (3, 1)],
            ],
            vec![
                [(1, 0), (2, 0), (0, 1), (1, 1), (1, 2)],
                [(1, 0), (0, 1), (1, 1), (2, 1), (2, 2)],
                [(1, 0), (1, 1), (2, 1), (0, 2), (1, 2)],
                [(0, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
                [(0, 0), (1, 0), (1, 1), (2, 1), (1, 2)],
                [(2, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
                [(1, 0), (0, 1), (1, 1), (1, 2), (2, 2)],
                [(1, 0), (0, 1), (1, 1), (2, 1), (0, 2)],
            ],
            vec![
                [(2, 0), (3, 0), (0, 1), (1, 1), (2, 1)],
                [(0, 0), (0, 1), (0, 2), (1, 2), (1, 3)],
                [(1, 0), (2, 0), (3, 0), (0, 1), (1, 1)],
                [(0, 0), (0, 1), (1, 1), (1, 2), (1, 3)],
                [(0, 0), (1, 0), (1, 1), (2, 1), (3, 1)],
                [(1, 0), (0, 1), (1, 1), (0, 2), (0, 3)],
                [(0, 0), (1, 0), (2, 0), (2, 1), (3, 1)],
                [(1, 0), (1, 1), (0, 2), (1, 2), (0, 3)],
            ],
            vec![
                [(0, 0), (1, 0), (2, 0), (1, 1), (1, 2)],
                [(2, 0), (0, 1), (1, 1), (2, 1), (2, 2)],
                [(1, 0), (1, 1), (0, 2), (1, 2), (2, 2)],
                [(0, 0), (0, 1), (1, 1), (2, 1), (0, 2)],
            ],
            vec![
                [(0, 0), (2, 0), (0, 1), (1, 1), (2, 1)],
                [(0, 0), (1, 0), (0, 1), (0, 2), (1, 2)],
                [(0, 0), (1, 0), (2, 0), (0, 1), (2, 1)],
                [(0, 0), (1, 0), (1, 1), (0, 2), (1, 2)],
            ],
            vec![
                [(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)],
                [(0, 0), (1, 0), (2, 0), (0, 1), (0, 2)],
                [(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
                [(2, 0), (2, 1), (0, 2), (1, 2), (2, 2)],
            ],
            vec![
                [(0, 0), (0, 1), (1, 1), (1, 2), (2, 2)],
                [(1, 0), (2, 0), (0, 1), (1, 1), (0, 2)],
                [(0, 0), (1, 0), (1, 1), (2, 1), (2, 2)],
                [(2, 0), (1, 1), (2, 1), (0, 2), (1, 2)],
            ],
            vec![[(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)]],
            vec![
                [(2, 0), (0, 1), (1, 1), (2, 1), (3, 1)],
                [(0, 0), (0, 1), (0, 2), (1, 2), (0, 3)],
                [(0, 0), (1, 0), (2, 0), (3, 0), (1, 1)],
                [(1, 0), (0, 1), (1, 1), (1, 2), (1, 3)],
                [(1, 0), (0, 1), (1, 1), (2, 1), (3, 1)],
                [(0, 0), (0, 1), (1, 1), (0, 2), (0, 3)],
                [(0, 0), (1, 0), (2, 0), (3, 0), (2, 1)],
                [(1, 0), (1, 1), (0, 2), (1, 2), (1, 3)],
            ],
            vec![
                [(0, 0), (1, 0), (1, 1), (1, 2), (2, 2)],
                [(2, 0), (0, 1), (1, 1), (2, 1), (0, 2)],
                [(1, 0), (2, 0), (1, 1), (0, 2), (1, 2)],
                [(0, 0), (0, 1), (1, 1), (2, 1), (2, 2)],
            ],
        ];
        let mut table = array::from_fn(|_| array::from_fn(|_| Vec::new()));
        for (n, shape) in shapes.iter().enumerate() {
            for s in shape {
                if s.iter().any(|p| p.0 + p.1 * cols >= rows * cols) {
                    continue;
                }
                let v = s.iter().map(|p| (1 << (p.0 + p.1 * cols))).sum::<u64>();
                let w = s.iter().map(|(x, _)| x).max().unwrap();
                let h = s.iter().map(|(_, y)| y).max().unwrap();
                for i in 0..cols.max(*w) - w {
                    for j in 0..rows.max(*h) - h {
                        let offset = i + j * cols;
                        table[s[0].0 + offset][n].push((v << offset).into());
                    }
                }
            }
        }
        Self { rows, cols, table }
    }
    fn solve(&self, initial: Bitboard) -> Vec<Vec<Bitboard>> {
        fn backtrack(
            current: Bitboard,
            remain: u32,
            table: &[[Vec<Bitboard>; 12]; 64],
            path: &mut Vec<Bitboard>,
            solutions: &mut Vec<Vec<Bitboard>>,
        ) {
            if remain == 0 {
                solutions.push(path.clone());
                return;
            }
            let target = u64::from(current).trailing_ones() as usize;
            for (i, candidate) in table[target].iter().enumerate() {
                if remain & (1 << i) != 0 {
                    for &c in candidate.iter() {
                        if (current & c).is_empty() {
                            path.push(c);
                            backtrack(current | c, remain & !(1 << i), table, path, solutions);
                            path.pop();
                        }
                    }
                }
            }
        }

        let mut solutions = Vec::new();
        backtrack(
            initial,
            (1 << NUM_PIECES) - 1,
            &self.table,
            &mut Vec::with_capacity(NUM_PIECES),
            &mut solutions,
        );
        solutions
    }
    fn represent_solution(&self, solution: &[Bitboard]) -> Vec<Vec<Option<Piece>>> {
        let mut ret = Vec::with_capacity(self.rows);
        for y in 0..self.rows {
            let mut row = Vec::with_capacity(self.cols);
            for x in 0..self.cols {
                let z = Bitboard::from(1 << (x + y * self.cols));
                if let Some(p) = solution.iter().find(|&p| !(*p & z).is_empty()) {
                    let a = self.table[u64::from(*p).trailing_zeros() as usize]
                        .iter()
                        .enumerate()
                        .find_map(|(i, v)| {
                            if v.contains(p) {
                                Piece::from_usize(i)
                            } else {
                                None
                            }
                        });
                    row.push(a);
                } else {
                    row.push(None);
                }
            }
            ret.push(row);
        }
        ret
    }
}
