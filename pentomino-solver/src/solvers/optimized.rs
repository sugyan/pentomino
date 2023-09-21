use super::Solver;
use crate::shapes::calculate_shapes;
use crate::{Bitboard, Piece, NUM_PIECES};
use num_traits::FromPrimitive;
use std::array;
use std::collections::HashMap;

const X_INDEX: usize = 9;

fn delta_swap(x: u64, (mask, delta): &(u64, u32)) -> u64 {
    let t = (x ^ (x >> delta)) & mask;
    x ^ t ^ (t << delta)
}

#[derive(Default)]
struct SolutionState {
    unique: bool,
    rows: usize,
    cols: usize,
    x_swaps: Vec<(u64, u32)>,
    y_swaps: Vec<(u64, u32)>,
    pieces: [Bitboard; NUM_PIECES],
    solutions: HashMap<[Bitboard; NUM_PIECES], bool>,
}

impl SolutionState {
    fn new(
        unique: bool,
        rows: usize,
        cols: usize,
        x_swaps: Vec<(u64, u32)>,
        y_swaps: Vec<(u64, u32)>,
    ) -> Self {
        Self {
            unique,
            rows,
            cols,
            x_swaps,
            y_swaps,
            ..Default::default()
        }
    }
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
    fn transpose(&self, pieces: &[Bitboard; NUM_PIECES]) -> [Bitboard; NUM_PIECES] {
        assert!(self.rows == 8 && self.cols == 8);
        array::from_fn(|i| {
            let mut u = u64::from(pieces[i]);
            u = delta_swap(u, &(0x00AA00AA00AA00AA, 7));
            u = delta_swap(u, &(0x0000CCCC0000CCCC, 14));
            u = delta_swap(u, &(0x00000000F0F0F0F0, 28));
            u.into()
        })
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
    x_swaps: Vec<(u64, u32)>,
    y_swaps: Vec<(u64, u32)>,
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
        let x_swaps = Self::generate_swaps((0..rows).map(|i| 1 << (cols * i)).sum(), cols, 1);
        let y_swaps = Self::generate_swaps((0..cols).map(|i| 1 << i).sum(), rows, cols);
        Self {
            rows,
            cols,
            transposed,
            table,
            xs,
            x_swaps,
            y_swaps,
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
        let mut state = SolutionState::new(
            unique,
            self.rows,
            self.cols,
            self.x_swaps.clone(),
            self.y_swaps.clone(),
        );
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
        let output = solver.x_swaps.iter().fold(input, delta_swap);
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
        let output = solver.y_swaps.iter().fold(input, delta_swap);
        assert_eq!(output, to_u64(&[46, 51, 55, 56, 57]), "{output:064b}");
    }

    fn to_u64(v: &[u32]) -> u64 {
        v.iter().map(|&i| 1 << i).sum()
    }
}
