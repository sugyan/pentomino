use super::Solver;
use crate::shapes::calculate_shapes;
use crate::{Bitboard, Piece, NUM_PIECES};
use num_traits::FromPrimitive;
use std::array;
use std::collections::HashSet;

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
}

#[derive(Default)]
struct SolutionState {
    unique: bool,
    limit: Option<usize>,
    remain: u32,
    pieces: [Bitboard; NUM_PIECES],
    solutions: Vec<[Bitboard; NUM_PIECES]>,
    set: HashSet<Board>,
}

impl SolutionState {
    fn new(unique: bool, limit: Option<usize>) -> Self {
        Self {
            unique,
            limit,
            remain: (1 << NUM_PIECES) - 1,
            ..Default::default()
        }
    }
    fn push(&mut self, i: usize, b: Bitboard) {
        self.pieces[i] = b;
        self.remain &= !(1 << i);
    }
    fn pop(&mut self, i: usize) {
        self.pieces[i] = Bitboard::default();
        self.remain |= 1 << i;
    }
    fn add_solution(&mut self, board: Board) -> bool {
        if self.unique {
            if !self.set.contains(&board) {
                self.solutions.push(self.pieces);
            }
            self.set.insert(board.flip_x());
            self.set.insert(board.flip_y());
            self.set.insert(board.flip_xy());
            if board.is_square() {
                self.set.insert(board.transpose());
            }
        } else {
            self.solutions.push(self.pieces);
        }
        self.limit.map_or(false, |l| self.solutions.len() >= l)
    }
}

pub struct SimpleSolver {
    rows: usize,
    cols: usize,
    table: [[Vec<Bitboard>; NUM_PIECES]; 64],
}

impl SimpleSolver {
    fn backtrack(&self, current: Bitboard, state: &mut SolutionState) -> bool {
        if state.remain == 0 {
            return state.add_solution(Board(self.represent_solution(&state.pieces)));
        }
        let target = u64::from(current).trailing_ones() as usize;
        for (i, candidate) in self.table[target].iter().enumerate() {
            if state.remain & (1 << i) != 0 {
                for &b in candidate.iter() {
                    if (current & b).is_empty() {
                        state.push(i, b);
                        if self.backtrack(current | b, state) {
                            return true;
                        }
                        state.pop(i);
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
    fn solve(
        &self,
        initial: Bitboard,
        unique: bool,
        limit: Option<usize>,
    ) -> Vec<[Bitboard; NUM_PIECES]> {
        let mut state = SolutionState::new(unique, limit);
        self.backtrack(initial, &mut state);
        state.solutions
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
