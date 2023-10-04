mod strategy;

use self::strategy::{LargeTableStrategy, SmallTableStrategy, Strategy};
use super::{SolutionStore, Solver};
use crate::{Bitboard, Piece, NUM_PIECES};
use num_traits::FromPrimitive;
use std::array;
use std::collections::{BTreeMap, BTreeSet};

const X_INDEX: usize = 9;

#[derive(Default)]
struct Transformer {
    x_swaps: Vec<(Bitboard, u32)>,
    y_swaps: Vec<(Bitboard, u32)>,
}

impl Transformer {
    fn new(rows: usize, cols: usize) -> Self {
        Self {
            x_swaps: Self::generate_swaps((0..rows).map(|i| 1 << (cols * i)).sum(), cols, 1),
            y_swaps: Self::generate_swaps((0..cols).map(|i| 1 << i).sum(), rows, cols),
        }
    }
    fn flip_x(&self, pieces: &[Bitboard; NUM_PIECES]) -> [Bitboard; NUM_PIECES] {
        array::from_fn(|i| self.x_swaps.iter().fold(pieces[i], Self::delta_swap))
    }
    fn flip_y(&self, pieces: &[Bitboard; NUM_PIECES]) -> [Bitboard; NUM_PIECES] {
        array::from_fn(|i| self.y_swaps.iter().fold(pieces[i], Self::delta_swap))
    }
    fn generate_swaps(unit: Bitboard, len: usize, steps: usize) -> Vec<(Bitboard, u32)> {
        let mut ret = Vec::new();
        let mut stack = vec![(vec![0], len)];
        while let Some((v, len)) = stack.last() {
            if *len < 2 {
                break;
            }
            let mut mask = 0;
            for i in v {
                for j in 0..*len / 2 {
                    mask |= unit << ((i + j) * steps);
                }
            }
            ret.push((mask, ((*len + 1) / 2 * steps) as u32));
            stack.push((
                v.iter().flat_map(|&i| [i, i + (*len + 1) / 2]).collect(),
                len / 2,
            ));
        }
        ret.reverse();
        ret
    }
    #[inline]
    fn delta_swap(x: Bitboard, (mask, delta): &(Bitboard, u32)) -> Bitboard {
        let t = (x ^ (x >> delta)) & mask;
        x ^ t ^ (t << delta)
    }
}

#[derive(Default)]
struct AllSolutionStore {
    solutions: BTreeSet<[Bitboard; NUM_PIECES]>,
    transformer: Transformer,
}

impl AllSolutionStore {
    fn new(transformer: Transformer) -> Self {
        Self {
            transformer,
            ..Default::default()
        }
    }
}

impl SolutionStore for AllSolutionStore {
    fn add_solution(&mut self, pieces: &[Bitboard; NUM_PIECES]) {
        let mut p = *pieces;
        self.solutions.insert(p);
        p = self.transformer.flip_x(&p);
        self.solutions.insert(p);
        p = self.transformer.flip_y(&p);
        self.solutions.insert(p);
        p = self.transformer.flip_x(&p);
        self.solutions.insert(p);
    }
    fn get_solutions(&self) -> Vec<[Bitboard; NUM_PIECES]> {
        self.solutions.iter().cloned().collect()
    }
}

#[derive(Default)]
struct UniqueSolutionStore<const SQ: bool> {
    solutions: BTreeMap<[Bitboard; NUM_PIECES], bool>,
    transformer: Transformer,
}

impl<const SQ: bool> UniqueSolutionStore<SQ> {
    fn new(transformer: Transformer) -> Self {
        Self {
            transformer,
            ..Default::default()
        }
    }
    fn transpose(pieces: &[Bitboard; NUM_PIECES]) -> [Bitboard; NUM_PIECES] {
        array::from_fn(|i| {
            let mut u = pieces[i];
            u = Transformer::delta_swap(u, &(0x00AA00AA00AA00AA, 7));
            u = Transformer::delta_swap(u, &(0x0000CCCC0000CCCC, 14));
            u = Transformer::delta_swap(u, &(0x00000000F0F0F0F0, 28));
            u
        })
    }
}

impl<const SQ: bool> SolutionStore for UniqueSolutionStore<SQ> {
    fn add_solution(&mut self, pieces: &[Bitboard; NUM_PIECES]) {
        self.solutions.entry(*pieces).or_insert(true);
        self.solutions
            .entry(self.transformer.flip_x(pieces))
            .or_insert(false);
        self.solutions
            .entry(self.transformer.flip_y(pieces))
            .or_insert(false);
        if SQ {
            self.solutions
                .entry(Self::transpose(pieces))
                .or_insert(false);
        }
    }
    fn get_solutions(&self) -> Vec<[Bitboard; NUM_PIECES]> {
        self.solutions
            .iter()
            .filter_map(|(k, v)| if *v { Some(k) } else { None })
            .cloned()
            .collect()
    }
}

pub enum OptimizedSolverType {
    SmallTable,
    LargeTable,
}

pub struct OptimizedSolver {
    rows: usize,
    cols: usize,
    transposed: bool,
    xs: Vec<Bitboard>,
    strategy: Box<dyn Strategy>,
}

impl OptimizedSolver {
    pub fn new(mut rows: usize, mut cols: usize, solver_type: OptimizedSolverType) -> Self {
        assert!(rows * cols <= 64);
        let transposed = if rows < cols {
            std::mem::swap(&mut rows, &mut cols);
            true
        } else {
            false
        };
        let mut xs = Vec::new();
        let v = [(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)]
            .iter()
            .map(|p| (1 << (p.0 + p.1 * cols)))
            .sum::<u64>();
        for y in 0..(rows - 1) / 2 {
            for x in 0..(cols - 1) / 2 {
                let offset = x + y * cols;
                if offset > 0 {
                    let u = v << offset;
                    xs.push(u);
                }
            }
        }
        let strategy: Box<dyn Strategy> = match solver_type {
            OptimizedSolverType::SmallTable => Box::new(SmallTableStrategy::new(rows, cols)),
            OptimizedSolverType::LargeTable => Box::new(LargeTableStrategy::new(rows, cols)),
        };
        Self {
            rows,
            cols,
            transposed,
            xs,
            strategy,
        }
    }
    fn execute<S: SolutionStore>(
        &self,
        initial: Bitboard,
        mut store: S,
    ) -> Vec<[Bitboard; NUM_PIECES]> {
        let mut pieces = [Bitboard::default(); NUM_PIECES];
        for x in &self.xs {
            if initial & x == 0 {
                pieces[X_INDEX] = *x;
                self.strategy
                    .backtrack(initial | x, 1 << X_INDEX, &mut pieces, &mut store);
            }
        }
        store.get_solutions()
    }
}

impl Solver for OptimizedSolver {
    fn solve(&self, initial: Bitboard, unique: bool) -> Vec<[Bitboard; NUM_PIECES]> {
        let transformer = Transformer::new(self.rows, self.cols);
        if unique {
            if self.rows == self.cols {
                self.execute(initial, UniqueSolutionStore::<true>::new(transformer))
            } else {
                self.execute(initial, UniqueSolutionStore::<false>::new(transformer))
            }
        } else {
            self.execute(initial, AllSolutionStore::new(transformer))
        }
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
                    if b & (1 << (x + y * self.cols)) != 0 {
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
