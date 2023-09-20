use super::Solver;
use crate::shapes::calculate_shapes;
use crate::{Bitboard, Piece, NUM_PIECES};
use num_traits::{FromPrimitive, ToPrimitive};
use std::array;
use std::collections::HashSet;

const X_INDEX: usize = 9;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Board(Vec<Vec<Option<Piece>>>);

impl Board {
    fn is_square(&self) -> bool {
        let (rows, cols) = (self.0.len(), self.0[0].len());
        rows == cols
    }
    fn flip_x(&self) -> Self {
        let mut ret = self.clone();
        for row in ret.0.iter_mut() {
            row.reverse();
        }
        ret
    }
    fn flip_y(&self) -> Self {
        let mut ret = self.clone();
        ret.0.reverse();
        ret
    }
    fn flip_xy(&self) -> Self {
        let mut ret = self.clone();
        for row in ret.0.iter_mut() {
            row.reverse();
        }
        ret.0.reverse();
        ret
    }
    fn transpose(&self) -> Self {
        let mut ret = self.clone();
        for (y, row) in ret.0.iter_mut().enumerate() {
            for (x, col) in row.iter_mut().enumerate() {
                *col = self.0[x][y];
            }
        }
        ret
    }
    fn to_pieces(&self, transposed: bool) -> [Bitboard; NUM_PIECES] {
        let (ros, cols) = (self.0.len(), self.0[0].len());
        let mut ret = [Bitboard::default(); NUM_PIECES];
        for (y, row) in self.0.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                if let Some(p) = col {
                    if let Some(i) = p.to_usize() {
                        let j = if transposed {
                            x * ros + y
                        } else {
                            x + y * cols
                        };
                        ret[i] |= Bitboard::from(1 << j);
                    }
                }
            }
        }
        ret
    }
}

#[derive(Default)]
struct SolutionState {
    unique: bool,
    pieces: [Bitboard; NUM_PIECES],
    solutions: Vec<[Bitboard; NUM_PIECES]>,
    set: HashSet<Board>,
}

impl SolutionState {
    fn new(unique: bool) -> Self {
        Self {
            unique,
            ..Default::default()
        }
    }
    fn add_solution(&mut self, board: Board, transposed: bool) {
        let x_flipped = board.flip_x();
        let y_flipped = board.flip_y();
        if self.unique {
            self.set.insert(x_flipped);
            self.set.insert(y_flipped);
            if board.is_square() {
                self.set.insert(board.transpose());
            }
        } else {
            for b in [x_flipped, y_flipped, board.flip_xy()] {
                if !self.set.contains(&b) {
                    self.solutions.push(b.to_pieces(transposed));
                    self.set.insert(b);
                }
            }
        }
        if !self.set.contains(&board) {
            self.solutions.push(self.pieces);
            self.set.insert(board);
        }
    }
}

pub struct OptimizedSolver {
    rows: usize,
    cols: usize,
    table: [Vec<Vec<(usize, Bitboard)>>; 64],
    transposed: bool,
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
            table,
            transposed,
            xs,
        }
    }
    fn backtrack(&self, current: Bitboard, remain: usize, state: &mut SolutionState) {
        if remain == 0 {
            return state.add_solution(
                Board(self.represent_solution(&state.pieces)),
                self.transposed,
            );
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
        let mut state = SolutionState::new(unique);
        let remain = ((1 << NUM_PIECES) - 1) & !(1 << X_INDEX);
        for &x in self.xs.iter().skip(1) {
            if (initial & x).is_empty() {
                state.pieces[X_INDEX] = x;
                self.backtrack(initial | x, remain, &mut state);
                state.pieces[X_INDEX] = Bitboard::default();
            }
        }
        state.solutions
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
