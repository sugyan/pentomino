use super::Solver;
use crate::shapes::calculate_shapes;
use crate::{Bitboard, Piece, NUM_PIECES};
use num_traits::FromPrimitive;
use std::array;
use std::collections::HashMap;

const X_INDEX: usize = 9;

#[derive(Default)]
struct SolutionState {
    unique: bool,
    rows: usize,
    cols: usize,
    pieces: [Bitboard; NUM_PIECES],
    solutions: HashMap<[Bitboard; NUM_PIECES], bool>,
}

impl SolutionState {
    fn new(unique: bool, rows: usize, cols: usize) -> Self {
        Self {
            unique,
            rows,
            cols,
            ..Default::default()
        }
    }
    fn flip_x(&self, pieces: &[Bitboard; NUM_PIECES]) -> [Bitboard; NUM_PIECES] {
        let mut ret = [Bitboard::default(); NUM_PIECES];
        for (i, b) in ret.iter_mut().enumerate() {
            for y in 0..self.rows {
                for x in 0..self.cols {
                    let j = x + y * self.cols;
                    let k = (self.cols - 1 - x) + y * self.cols;
                    if !(pieces[i] & Bitboard::from(1 << j)).is_empty() {
                        *b |= Bitboard::from(1 << k);
                    }
                }
            }
        }
        ret
    }
    fn flip_y(&self, pieces: &[Bitboard; NUM_PIECES]) -> [Bitboard; NUM_PIECES] {
        let mut ret = [Bitboard::default(); NUM_PIECES];
        for (i, b) in ret.iter_mut().enumerate() {
            for y in 0..self.rows {
                for x in 0..self.cols {
                    let j = x + y * self.cols;
                    let k = x + (self.rows - 1 - y) * self.cols;
                    if !(pieces[i] & Bitboard::from(1 << j)).is_empty() {
                        *b |= Bitboard::from(1 << k);
                    }
                }
            }
        }
        ret
    }
    fn transpose(&self, pieces: &[Bitboard; NUM_PIECES]) -> [Bitboard; NUM_PIECES] {
        let mut ret = [Bitboard::default(); NUM_PIECES];
        for (i, b) in ret.iter_mut().enumerate() {
            for y in 0..self.rows {
                for x in 0..self.cols {
                    let j = x + y * self.cols;
                    let k = y + x * self.rows;
                    if !(pieces[i] & Bitboard::from(1 << j)).is_empty() {
                        *b |= Bitboard::from(1 << k);
                    }
                }
            }
        }
        ret
    }
    fn add_solution(&mut self) {
        let x_flipped = self.flip_x(&self.pieces);
        let y_flipped = self.flip_y(&self.pieces);
        if self.unique {
            self.solutions.entry(x_flipped).or_insert(false);
            self.solutions.entry(y_flipped).or_insert(false);
            if self.rows == self.cols {
                self.solutions
                    .entry(self.transpose(&self.pieces))
                    .or_insert(false);
            }
        } else {
            let xy_flipped = self.flip_x(&y_flipped);
            for b in [x_flipped, y_flipped, xy_flipped] {
                self.solutions.entry(b).or_insert(true);
            }
        }
        self.solutions.entry(self.pieces).or_insert(true);
    }
}

pub struct OptimizedSolver {
    rows: usize,
    cols: usize,
    transposed: bool,
    table: [Vec<Vec<(usize, Bitboard)>>; 64],
    xs: Vec<Bitboard>,
}

impl OptimizedSolver {
    pub fn new(mut rows: usize, mut cols: usize) -> Self {
        assert!(rows * cols <= 64);
        let transposed = if rows < cols {
            std::mem::swap(&mut rows, &mut cols);
            true
        } else {
            false
        };
        let shapes = calculate_shapes();
        let mut table = array::from_fn(|_| vec![Vec::new(); 1 << NUM_PIECES]);
        let mut xs = Vec::new();
        for (n, shape) in shapes.iter().enumerate() {
            for s in shape {
                if s.iter().any(|&(x, y)| x >= cols || y >= rows) {
                    continue;
                }
                let v = s.iter().map(|p| (1 << (p.0 + p.1 * cols))).sum::<u64>();
                let (w, h) = s
                    .iter()
                    .fold((0, 0), |(xmax, ymax), &(x, y)| (xmax.max(x), ymax.max(y)));
                for y in 0..rows - h {
                    for x in 0..cols - w {
                        let offset = x + y * cols;
                        for i in 0..(1 << NUM_PIECES) {
                            if (i & (1 << n)) != 0 {
                                table[s[0].0 + offset][i].push((n, (v << offset).into()));
                            }
                        }
                        // X
                        if n == X_INDEX && x < (cols - 1) / 2 && y < (rows - 1) / 2 {
                            xs.push((v << offset).into());
                        }
                    }
                }
            }
        }
        Self {
            rows,
            cols,
            transposed,
            table,
            xs,
        }
    }
    fn backtrack(&self, current: Bitboard, remain: usize, state: &mut SolutionState) {
        if remain == 0 {
            return state.add_solution();
        }
        let target = u64::from(current).trailing_ones() as usize;
        for &(i, b) in &self.table[target][remain] {
            if (current & b).is_empty() {
                state.pieces[i] = b;
                self.backtrack(current | b, remain & !(1 << i), state);
                state.pieces[i] = Bitboard::default();
            }
        }
    }
}

impl Solver for OptimizedSolver {
    fn solve(&self, initial: Bitboard, unique: bool) -> Vec<[Bitboard; NUM_PIECES]> {
        let mut state = SolutionState::new(unique, self.rows, self.cols);
        let remain = ((1 << NUM_PIECES) - 1) & !(1 << X_INDEX);
        for &x in self.xs.iter().skip(1) {
            if (initial & x).is_empty() {
                state.pieces[X_INDEX] = x;
                self.backtrack(initial | x, remain, &mut state);
                state.pieces[X_INDEX] = Bitboard::default();
            }
        }
        state
            .solutions
            .into_iter()
            .filter_map(|(k, v)| if v { Some(k) } else { None })
            .collect()
    }
    fn represent_solution(&self, solution: &[Bitboard; NUM_PIECES]) -> Vec<Vec<Option<Piece>>> {
        let mut ret = if self.transposed {
            vec![vec![None; self.rows]; self.cols]
        } else {
            vec![vec![None; self.cols]; self.rows]
        };
        for (i, b) in solution.iter().enumerate() {
            let p = Piece::from_usize(i);
            for y in 0..self.rows {
                for x in 0..self.cols {
                    if !(*b & Bitboard::from(1 << (x + y * self.cols))).is_empty() {
                        if self.transposed {
                            ret[x][y] = p;
                        } else {
                            ret[y][x] = p;
                        }
                    }
                }
            }
        }
        ret
    }
}
