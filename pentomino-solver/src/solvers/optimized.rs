use super::Solver;
use crate::shapes::calculate_shapes;
use crate::{Bitboard, Piece, NUM_PIECES};
use num_traits::FromPrimitive;
use std::array;
use std::collections::{BTreeMap, BTreeSet};

const X_INDEX: usize = 9;

#[inline]
fn delta_swap(x: u64, (mask, delta): &(u64, u32)) -> u64 {
    let t = (x ^ (x >> delta)) & mask;
    x ^ t ^ (t << delta)
}

trait SolutionStore {
    fn add_solution(&mut self, pieces: &[[Bitboard; NUM_PIECES]; 4]);
    fn get_solutions(self) -> Vec<[Bitboard; NUM_PIECES]>;
}

#[derive(Default)]
struct AllSolutionStore {
    solutions: BTreeSet<[Bitboard; NUM_PIECES]>,
}

impl SolutionStore for AllSolutionStore {
    fn add_solution(&mut self, pieces: &[[Bitboard; NUM_PIECES]; 4]) {
        for p in pieces {
            self.solutions.insert(*p);
        }
    }
    fn get_solutions(self) -> Vec<[Bitboard; NUM_PIECES]> {
        self.solutions.into_iter().collect()
    }
}

#[derive(Default)]
struct UniqueSolutionStore<const SQ: bool> {
    solutions: BTreeMap<[Bitboard; NUM_PIECES], bool>,
}

impl<const SQ: bool> UniqueSolutionStore<SQ> {
    fn transpose(pieces: &[Bitboard; NUM_PIECES]) -> [Bitboard; NUM_PIECES] {
        array::from_fn(|i| {
            let mut u = u64::from(pieces[i]);
            u = delta_swap(u, &(0x00AA00AA00AA00AA, 7));
            u = delta_swap(u, &(0x0000CCCC0000CCCC, 14));
            u = delta_swap(u, &(0x00000000F0F0F0F0, 28));
            u.into()
        })
    }
}

impl<const SQ: bool> SolutionStore for UniqueSolutionStore<SQ> {
    fn add_solution(&mut self, pieces: &[[Bitboard; NUM_PIECES]; 4]) {
        self.solutions.entry(pieces[0]).or_insert(true);
        self.solutions.entry(pieces[1]).or_insert(false);
        self.solutions.entry(pieces[2]).or_insert(false);
        if SQ {
            self.solutions
                .entry(Self::transpose(&pieces[0]))
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

pub struct OptimizedSolver {
    rows: usize,
    cols: usize,
    transposed: bool,
    table: Vec<Vec<(usize, [Bitboard; 4])>>,
    xs: Vec<[Bitboard; 4]>,
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
        let mut table = vec![Vec::new(); (1 << NUM_PIECES) * 64];
        let mut xs = Vec::new();
        let edge_x = (0..cols).map(|i| 1 << i).sum::<u64>();
        let edge_y = (0..rows).map(|i| 1 << (i * cols)).sum::<u64>();
        let edges = [
            edge_x,
            edge_y,
            edge_x << ((rows - 1) * cols),
            edge_y << (cols - 1),
        ];
        let x_swaps = Self::generate_swaps((0..rows).map(|i| 1 << (cols * i)).sum(), cols, 1);
        let y_swaps = Self::generate_swaps((0..cols).map(|i| 1 << i).sum(), rows, cols);
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
                        let u0 = v << offset;
                        if [0, 1, 2, 3, 0].windows(2).any(|w| {
                            let (e0, e1) = (edges[w[0]], edges[w[1]]);
                            (e0 & u0) != 0 && (e1 & u0) != 0 && (e0 & e1) & u0 == 0
                        }) {
                            continue;
                        }
                        let u1 = x_swaps.iter().fold(u0, delta_swap);
                        let u2 = y_swaps.iter().fold(u0, delta_swap);
                        let u3 = x_swaps.iter().fold(u2, delta_swap);
                        let bs = [u0.into(), u1.into(), u2.into(), u3.into()];
                        for i in 0..(1 << NUM_PIECES) {
                            if (i & (1 << n)) == 0 {
                                table[((s[0].0 + offset) << NUM_PIECES) + i].push((n, bs));
                            }
                        }
                        // X
                        if n == X_INDEX && x < (cols - 1) / 2 && y < (rows - 1) / 2 {
                            xs.push(bs);
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
    fn execute<S: SolutionStore>(
        &self,
        initial: Bitboard,
        mut store: S,
    ) -> Vec<[Bitboard; NUM_PIECES]> {
        let mut pieces = [[Bitboard::default(); NUM_PIECES]; 4];
        for &xs in &self.xs {
            if (initial & xs[0]).is_empty() {
                for j in 0..4 {
                    pieces[j][X_INDEX] = xs[j];
                }
                self.backtrack(initial | xs[0], 1 << X_INDEX, &mut pieces, &mut store);
            }
        }
        store.get_solutions()
    }
    fn backtrack<S: SolutionStore>(
        &self,
        current: Bitboard,
        used: u32,
        pieces: &mut [[Bitboard; NUM_PIECES]; 4],
        store: &mut S,
    ) {
        if used == (1 << NUM_PIECES) - 1 {
            return store.add_solution(pieces);
        }
        let target = ((u64::from(current).trailing_ones() << NUM_PIECES) + used) as usize;
        for &(i, bs) in &self.table[target] {
            if (current & bs[0]).is_empty() {
                for j in 0..4 {
                    pieces[j][i] = bs[j];
                }
                self.backtrack(current | bs[0], used | (1 << i), pieces, store);
            }
        }
    }
    fn generate_swaps(unit: u64, len: usize, steps: usize) -> Vec<(u64, u32)> {
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
}

impl Solver for OptimizedSolver {
    fn solve(&self, initial: Bitboard, unique: bool) -> Vec<[Bitboard; NUM_PIECES]> {
        if unique {
            if self.rows == self.cols {
                self.execute(initial, UniqueSolutionStore::<true>::default())
            } else {
                self.execute(initial, UniqueSolutionStore::<false>::default())
            }
        } else {
            self.execute(initial, AllSolutionStore::default())
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
