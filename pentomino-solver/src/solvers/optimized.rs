use super::{SolutionStore, Solver};
use crate::shapes::calculate_shapes;
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
    fn get_solutions(self) -> Vec<[Bitboard; NUM_PIECES]> {
        self.solutions.into_iter().collect()
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
    fn get_solutions(self) -> Vec<[Bitboard; NUM_PIECES]> {
        self.solutions
            .into_iter()
            .filter_map(|(k, v)| if v { Some(k) } else { None })
            .collect()
    }
}

type Table = [Vec<Vec<(usize, Bitboard)>>; 64];

struct TableGenerator {
    rows: usize,
    cols: usize,
    edges: [Bitboard; 4],
    shapes: Vec<Vec<Vec<(usize, usize)>>>,
    holes: Vec<(Bitboard, Bitboard)>,
}

impl TableGenerator {
    fn new(rows: usize, cols: usize) -> Self {
        let edge_x = (0..cols).map(|i| 1 << i).sum::<u64>();
        let edge_y = (0..rows).map(|i| 1 << (i * cols)).sum::<u64>();
        let mut holes = Vec::with_capacity(rows * cols);
        for y in 0..rows {
            for x in 0..cols {
                let mut u = 0;
                for (dx, dy) in [(0, !0), (0, 1), (!0, 0), (1, 0)] {
                    let (x, y) = (x.wrapping_add(dx), y.wrapping_add(dy));
                    if (0..cols).contains(&x) && (0..rows).contains(&y) {
                        u |= 1 << (x + y * cols);
                    }
                }
                holes.push((u | (1 << (x + y * cols)), u));
            }
        }
        Self {
            rows,
            cols,
            edges: [
                edge_x,
                edge_y,
                edge_x << ((rows - 1) * cols),
                edge_y << (cols - 1),
            ],
            shapes: calculate_shapes(),
            holes,
        }
    }
    fn generate_tables(&self) -> Vec<(Bitboard, Table)> {
        let xs = self.generate_xs();
        let mut tables = vec![array::from_fn(|_| vec![Vec::new(); 1 << NUM_PIECES]); xs.len()];
        for (i, shape) in self.shapes.iter().enumerate() {
            if i == X_INDEX {
                continue;
            }
            for s in shape {
                if s.iter().any(|&(x, y)| x >= self.cols || y >= self.rows) {
                    continue;
                }
                let v = s
                    .iter()
                    .map(|p| (1 << (p.0 + p.1 * self.cols)))
                    .sum::<u64>();
                let (w, h) = s
                    .iter()
                    .fold((0, 0), |(xmax, ymax), &(x, y)| (xmax.max(x), ymax.max(y)));
                for y in 0..self.rows - h {
                    for x in 0..self.cols - w {
                        let offset = x + y * self.cols;
                        let u = v << offset;
                        if self.check_corner_space(u) {
                            continue;
                        }
                        for (j, &b) in xs.iter().enumerate() {
                            if (u & b) != 0 || self.check_hole(u | b) {
                                continue;
                            }
                            for k in 0..(1 << NUM_PIECES) {
                                if (k & (1 << i)) == 0 {
                                    tables[j][s[0].0 + offset][k].push((i, u));
                                }
                            }
                        }
                    }
                }
            }
        }
        xs.into_iter().zip(tables).collect()
    }
    fn generate_xs(&self) -> Vec<Bitboard> {
        let mut xs = Vec::new();
        assert!(self.shapes[X_INDEX].len() == 1);
        let v = &self.shapes[X_INDEX][0]
            .iter()
            .map(|p| (1 << (p.0 + p.1 * self.cols)))
            .sum::<u64>();
        for y in 0..(self.rows - 1) / 2 {
            for x in 0..(self.cols - 1) / 2 {
                let offset = x + y * self.cols;
                if offset > 0 {
                    let u = v << offset;
                    xs.push(u);
                }
            }
        }
        xs
    }
    fn check_corner_space(&self, u: Bitboard) -> bool {
        [0, 1, 2, 3, 0].windows(2).any(|w| {
            let (e0, e1) = (self.edges[w[0]], self.edges[w[1]]);
            (e0 & u) != 0 && (e1 & u) != 0 && (e0 & e1) & u == 0
        })
    }
    fn check_hole(&self, u: Bitboard) -> bool {
        self.holes.iter().any(|&(mask, hole)| (u & mask) == hole)
    }
}

pub struct OptimizedSolver {
    rows: usize,
    cols: usize,
    transposed: bool,
    tables: Vec<(Bitboard, Table)>,
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
        let tables = TableGenerator::new(rows, cols).generate_tables();
        Self {
            rows,
            cols,
            transposed,
            tables,
        }
    }
    fn execute<S: SolutionStore>(
        &self,
        initial: Bitboard,
        mut store: S,
    ) -> Vec<[Bitboard; NUM_PIECES]> {
        let mut pieces = [Bitboard::default(); NUM_PIECES];
        for (x, table) in &self.tables {
            if initial & x == 0 {
                pieces[X_INDEX] = *x;
                Self::backtrack(initial | *x, 1 << X_INDEX, table, &mut pieces, &mut store);
            }
        }
        store.get_solutions()
    }
    fn backtrack<S: SolutionStore>(
        current: Bitboard,
        used: usize,
        table: &Table,
        pieces: &mut [Bitboard; NUM_PIECES],
        store: &mut S,
    ) {
        if used == (1 << NUM_PIECES) - 1 {
            return store.add_solution(pieces);
        }
        let target = current.trailing_ones() as usize;
        for &(i, b) in &table[target][used] {
            if current & b == 0 {
                pieces[i] = b;
                Self::backtrack(current | b, used | (1 << i), table, pieces, store);
            }
        }
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
