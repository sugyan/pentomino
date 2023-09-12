use crate::{Piece, Solver, NUM_PIECES};
use num_traits::FromPrimitive;
use std::array;

pub struct SimpleSolver {
    rows: usize,
    cols: usize,
    table: [[Vec<u64>; NUM_PIECES]; 64],
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
                let v = s.iter().map(|p| (1 << (p.0 + p.1 * cols))).sum::<u64>();
                let w = s.iter().map(|(x, _)| x).max().unwrap();
                let h = s.iter().map(|(_, y)| y).max().unwrap();
                for i in 0..cols - w {
                    for j in 0..rows - h {
                        let offset = i + j * cols;
                        table[s[0].0 + offset][n].push(v << offset);
                    }
                }
            }
        }
        Self { rows, cols, table }
    }
    fn solve(&self, start: u64) -> Vec<Vec<u64>> {
        fn backtrack(
            current: u64,
            remain: u32,
            table: &[[Vec<u64>; 12]; 64],
            path: &mut Vec<u64>,
            solutions: &mut Vec<Vec<u64>>,
        ) {
            if remain == 0 {
                solutions.push(path.clone());
                return;
            }
            let target = current.trailing_ones() as usize;
            for (i, candidate) in table[target].iter().enumerate() {
                if remain & (1 << i) != 0 {
                    for &c in candidate.iter() {
                        if current & c == 0 {
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
            start,
            (1 << NUM_PIECES) - 1,
            &self.table,
            &mut Vec::with_capacity(NUM_PIECES),
            &mut solutions,
        );
        solutions
    }
    fn represent_solution(&self, solution: &[u64]) -> Option<Vec<Vec<Option<Piece>>>> {
        let mut ret = Vec::with_capacity(self.rows);
        for y in 0..self.rows {
            let mut row = Vec::with_capacity(self.cols);
            for x in 0..self.cols {
                let z = 1 << (x + y * self.cols);
                if let Some(p) = solution.iter().find(|&p| p & z != 0) {
                    let i = self.table[p.trailing_zeros() as usize]
                        .iter()
                        .enumerate()
                        .find_map(|(i, v)| if v.contains(p) { Some(i) } else { None })?;
                    row.push(Some(Piece::from_usize(i))?);
                } else {
                    row.push(None);
                }
            }
            ret.push(row);
        }
        Some(ret)
    }
}
