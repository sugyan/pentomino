use super::{SolutionStore, Solver};
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
struct AllSolutionStore {
    solutions: Vec<[Bitboard; NUM_PIECES]>,
}

impl SolutionStore for AllSolutionStore {
    fn add_solution(&mut self, pieces: &[Bitboard; NUM_PIECES]) {
        self.solutions.push(*pieces);
    }
    fn get_solutions(&self) -> Vec<[Bitboard; NUM_PIECES]> {
        self.solutions.clone()
    }
}

type Converter<'a> = Box<dyn Fn(&[Bitboard; NUM_PIECES]) -> Board + 'a>;

struct UniqueSolutionStore<'a> {
    converter: Converter<'a>,
    solutions: Vec<[Bitboard; NUM_PIECES]>,
    set: HashSet<Board>,
}

impl<'a> UniqueSolutionStore<'a> {
    fn new(converter: impl Fn(&[Bitboard; NUM_PIECES]) -> Board + 'a) -> Self {
        Self {
            converter: Box::new(converter),
            solutions: Vec::new(),
            set: HashSet::new(),
        }
    }
}

impl<'a> SolutionStore for UniqueSolutionStore<'a> {
    fn add_solution(&mut self, pieces: &[Bitboard; NUM_PIECES]) {
        let board = (self.converter)(pieces);
        if !self.set.contains(&board) {
            self.solutions.push(*pieces);
        }
        self.set.insert(board.flip_x());
        self.set.insert(board.flip_y());
        self.set.insert(board.flip_xy());
        if board.is_square() {
            self.set.insert(board.transpose());
        }
    }
    fn get_solutions(&self) -> Vec<[Bitboard; NUM_PIECES]> {
        self.solutions.clone()
    }
}

pub struct DefaultSolver {
    rows: usize,
    cols: usize,
    table: [[Vec<Bitboard>; NUM_PIECES]; 64],
}

impl DefaultSolver {
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
                for y in 0..rows - h {
                    for x in 0..cols - w {
                        let offset = x + y * cols;
                        table[s[0].0 + offset][n].push(v << offset);
                    }
                }
            }
        }
        Self { rows, cols, table }
    }
    fn execute<S: SolutionStore>(
        &self,
        initial: Bitboard,
        mut store: S,
    ) -> Vec<[Bitboard; NUM_PIECES]> {
        self.backtrack(
            initial,
            (1 << NUM_PIECES) - 1,
            &mut [Bitboard::default(); NUM_PIECES],
            &mut store,
        );
        store.get_solutions()
    }
    fn backtrack<S: SolutionStore>(
        &self,
        current: Bitboard,
        remain: u32,
        pieces: &mut [Bitboard; NUM_PIECES],
        store: &mut S,
    ) {
        if remain == 0 {
            return store.add_solution(pieces);
        }
        let target = current.trailing_ones() as usize;
        for (i, candidates) in self.table[target].iter().enumerate() {
            if remain & (1 << i) != 0 {
                for &b in candidates.iter() {
                    if current & b == 0 {
                        pieces[i] = b;
                        self.backtrack(current | b, remain & !(1 << i), pieces, store);
                        pieces[i] = Bitboard::default();
                    }
                }
            }
        }
    }
}

impl Solver for DefaultSolver {
    fn solve(&self, initial: Bitboard, unique: bool) -> Vec<[Bitboard; NUM_PIECES]> {
        if unique {
            self.execute(
                initial,
                UniqueSolutionStore::new(|pieces: &[Bitboard; NUM_PIECES]| {
                    Board(self.represent_solution(pieces))
                }),
            )
        } else {
            self.execute(initial, AllSolutionStore::default())
        }
    }
    fn represent_solution(&self, solution: &[Bitboard; NUM_PIECES]) -> Vec<Vec<Option<Piece>>> {
        let mut ret = vec![vec![None; self.cols]; self.rows];
        for (i, b) in solution.iter().enumerate() {
            let p = Piece::from_usize(i);
            for (y, row) in ret.iter_mut().enumerate() {
                for (x, col) in row.iter_mut().enumerate() {
                    if b & (1 << (x + y * self.cols)) != 0 {
                        *col = p;
                    }
                }
            }
        }
        ret
    }
}
