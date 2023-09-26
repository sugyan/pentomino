use super::{SolutionStore, Solver};
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

#[derive(Default, Clone)]
struct Transformer {
    x_swaps: Vec<(u64, u32)>,
    y_swaps: Vec<(u64, u32)>,
}

impl Transformer {
    fn flip_x(&self, pieces: &[Bitboard; NUM_PIECES]) -> [Bitboard; NUM_PIECES] {
        array::from_fn(|i| {
            self.x_swaps
                .iter()
                .fold(u64::from(pieces[i]), delta_swap)
                .into()
        })
    }
    fn flip_y(&self, pieces: &[Bitboard; NUM_PIECES]) -> [Bitboard; NUM_PIECES] {
        array::from_fn(|i| {
            self.y_swaps
                .iter()
                .fold(u64::from(pieces[i]), delta_swap)
                .into()
        })
    }
}

#[derive(Default)]
struct AllSolutionStore {
    transformer: Transformer,
    solutions: BTreeSet<[Bitboard; NUM_PIECES]>,
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
        let pieces = *pieces;
        self.solutions.insert(pieces);
        let pieces = self.transformer.flip_x(&pieces);
        self.solutions.insert(pieces);
        let pieces = self.transformer.flip_y(&pieces);
        self.solutions.insert(pieces);
        let pieces = self.transformer.flip_x(&pieces);
        self.solutions.insert(pieces);
    }
    fn get_solutions(self) -> Vec<[Bitboard; NUM_PIECES]> {
        self.solutions.into_iter().collect()
    }
}

#[derive(Default)]
struct UniqueSolutionStore<const SQ: bool> {
    transformer: Transformer,
    solutions: BTreeMap<[Bitboard; NUM_PIECES], bool>,
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
            let mut u = u64::from(pieces[i]);
            u = delta_swap(u, &(0x00AA00AA00AA00AA, 7));
            u = delta_swap(u, &(0x0000CCCC0000CCCC, 14));
            u = delta_swap(u, &(0x00000000F0F0F0F0, 28));
            u.into()
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

pub struct OptimizedSolver {
    rows: usize,
    cols: usize,
    transposed: bool,
    table: [Vec<Vec<(usize, Bitboard)>>; 64],
    xs: Vec<Bitboard>,
    transformer: Transformer,
    holes: [(Bitboard, Bitboard); 64],
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
        let transformer = Transformer {
            x_swaps: Self::generate_swaps((0..rows).map(|i| 1 << (cols * i)).sum(), cols, 1),
            y_swaps: Self::generate_swaps((0..cols).map(|i| 1 << i).sum(), rows, cols),
        };
        let mut holes = [(Bitboard::default(), Bitboard::from(!0)); 64];
        for y in 0..rows {
            for x in 0..cols {
                let z = x + y * cols;
                let mut v = 0_u64;
                for (dx, dy) in [(!0, 0), (0, !0), (1, 0), (0, 1)] {
                    let x = x.wrapping_add(dx);
                    let y = y.wrapping_add(dy);
                    if (0..cols).contains(&x) && (0..rows).contains(&y) {
                        v |= 1 << (x + y * cols);
                    }
                }
                let (mask, check) = ((v | (1 << z)).into(), v.into());
                if y > 0 && x < cols - 1 {
                    holes[(x + 1) + (y - 1) * cols] = (mask, check);
                    if x == 0 {
                        holes[x + (y - 1) * cols] = (mask, check);
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
            transformer,
            holes,
        }
    }
    fn execute<S: SolutionStore>(
        &self,
        initial: Bitboard,
        mut store: S,
    ) -> Vec<[Bitboard; NUM_PIECES]> {
        let remain = ((1 << NUM_PIECES) - 1) & !(1 << X_INDEX);
        let mut pieces = [Bitboard::default(); NUM_PIECES];
        for &x in self.xs.iter().skip(1) {
            if (initial & x).is_empty() {
                pieces[X_INDEX] = x;
                self.backtrack(initial | x, remain, &mut pieces, &mut store);
                pieces[X_INDEX] = Bitboard::default();
            }
        }
        store.get_solutions()
    }
    fn backtrack<S: SolutionStore>(
        &self,
        current: Bitboard,
        remain: usize,
        pieces: &mut [Bitboard; NUM_PIECES],
        store: &mut S,
    ) {
        if remain == 0 {
            return store.add_solution(pieces);
        }
        let target = u64::from(current).trailing_ones() as usize;
        for &(i, b) in &self.table[target][remain] {
            if (current & b).is_empty() {
                let next = current | b;
                if next & self.holes[target].0 == self.holes[target].1 {
                    continue;
                }
                pieces[i] = b;
                self.backtrack(next, remain & !(1 << i), pieces, store);
                pieces[i] = Bitboard::default();
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
                self.execute(
                    initial,
                    UniqueSolutionStore::<true>::new(self.transformer.clone()),
                )
            } else {
                self.execute(
                    initial,
                    UniqueSolutionStore::<false>::new(self.transformer.clone()),
                )
            }
        } else {
            self.execute(initial, AllSolutionStore::new(self.transformer.clone()))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flip_x_by_delta_swap() {
        let solver = OptimizedSolver::new(5, 12);
        assert_eq!(solver.rows, 12);
        assert_eq!(solver.cols, 5);
        assert!(solver.transposed);

        // ###..    ..###
        // .#...    ...#.
        // .#...    ...#.
        // .....    .....
        //
        // ...   -> ...
        //
        // .....    .....
        // .....    .....
        // .....    .....
        // .....    .....
        let input = to_u64(&[0, 1, 2, 6, 11]);
        let output = solver.transformer.x_swaps.iter().fold(input, delta_swap);
        assert_eq!(output, to_u64(&[2, 3, 4, 8, 13]), "{output:064b}");
    }

    #[test]
    fn flip_y_by_delta_swap() {
        let solver = OptimizedSolver::new(5, 12);
        assert_eq!(solver.rows, 12);
        assert_eq!(solver.cols, 5);
        assert!(solver.transposed);

        // ###..    .....
        // .#...    .....
        // .#...    .....
        // .....    .....
        //
        // ...   ->  ...
        //
        // .....    .....
        // .....    .#...
        // .....    .#...
        // .....    ###..
        let input = to_u64(&[0, 1, 2, 6, 11]);
        let output = solver.transformer.y_swaps.iter().fold(input, delta_swap);
        assert_eq!(output, to_u64(&[46, 51, 55, 56, 57]), "{output:064b}");
    }

    fn to_u64(v: &[u32]) -> u64 {
        v.iter().map(|&i| 1 << i).sum()
    }
}
