use super::X_INDEX;
use crate::solvers::SolutionStore;
use crate::{Bitboard, NUM_PIECES};
use std::array;

type HoleCheckers = [[(Bitboard, Bitboard); 2]; 64];

fn hole_checkers(rows: usize, cols: usize) -> HoleCheckers {
    let mut h = Vec::new();
    for y in 0..rows {
        for x in 0..cols {
            let mut u = 0;
            for (dx, dy) in [(0, !0), (0, 1), (!0, 0), (1, 0)] {
                let (x, y) = (x.wrapping_add(dx), y.wrapping_add(dy));
                if (0..cols).contains(&x) && (0..rows).contains(&y) {
                    u |= 1 << (x + y * cols);
                }
            }
            h.push((u | (1 << (x + y * cols)), u));
        }
    }
    array::from_fn(|i| [h[(i + 1) % h.len()], h[(i + cols - 1) % h.len()]])
}

pub(super) trait Strategy {
    fn new(rows: usize, cols: usize) -> Self
    where
        Self: Sized;
    fn backtrack(
        &self,
        current: Bitboard,
        used: usize,
        pieces: &mut [Bitboard; NUM_PIECES],
        store: &mut dyn SolutionStore,
    );
}

pub(super) struct SmallTableStrategy {
    table: [[Vec<Bitboard>; NUM_PIECES]; 64],
    holes: HoleCheckers,
}

impl Strategy for SmallTableStrategy {
    fn new(rows: usize, cols: usize) -> Self {
        let mut table = array::from_fn(|_| array::from_fn(|_| Vec::new()));
        let checker = Checker::new(rows, cols);
        let shapes = crate::shapes::calculate_shapes();
        for (i, shape) in shapes.iter().enumerate() {
            if i == X_INDEX {
                continue;
            }
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
                        let u = v << offset;
                        if checker.check_corner_space(u) || checker.check_hole(u) {
                            continue;
                        }
                        table[s[0].0 + offset][i].push(u);
                    }
                }
            }
        }
        Self {
            table,
            holes: hole_checkers(rows, cols),
        }
    }
    fn backtrack(
        &self,
        current: Bitboard,
        used: usize,
        pieces: &mut [Bitboard; NUM_PIECES],
        store: &mut dyn SolutionStore,
    ) {
        if used == (1 << NUM_PIECES) - 1 {
            return store.add_solution(pieces);
        }
        let target = current.trailing_ones() as usize;

        let mut u = !used & ((1 << NUM_PIECES) - 1);
        while u != 0 {
            let i = u.trailing_zeros() as usize;
            let used = used | (1 << i);
            for b in &self.table[target][i] {
                if current & b == 0 {
                    let next = current | b;
                    if self.holes[target].iter().any(|&(u, v)| next & u == v) {
                        continue;
                    }
                    pieces[i] = *b;
                    self.backtrack(next, used, pieces, store);
                }
            }
            u &= u - 1;
        }
    }
}

pub(super) struct LargeTableStrategy {
    table: [Vec<Vec<(usize, Bitboard)>>; 64],
    holes: HoleCheckers,
}

impl Strategy for LargeTableStrategy {
    fn new(rows: usize, cols: usize) -> Self {
        let mut table = array::from_fn(|_| vec![Vec::new(); 1 << NUM_PIECES]);
        let checker = Checker::new(rows, cols);
        let shapes = crate::shapes::calculate_shapes();
        for (i, shape) in shapes.iter().enumerate() {
            if i == X_INDEX {
                continue;
            }
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
                        let u = v << offset;
                        if checker.check_corner_space(u) || checker.check_hole(u) {
                            continue;
                        }
                        for j in 0..(1 << NUM_PIECES) {
                            if (j & (1 << i)) == 0 {
                                table[s[0].0 + offset][j].push((i, u));
                            }
                        }
                    }
                }
            }
        }
        Self {
            table,
            holes: hole_checkers(rows, cols),
        }
    }
    fn backtrack(
        &self,
        current: Bitboard,
        used: usize,
        pieces: &mut [Bitboard; NUM_PIECES],
        store: &mut dyn SolutionStore,
    ) {
        if used == (1 << NUM_PIECES) - 1 {
            return store.add_solution(pieces);
        }
        let target = current.trailing_ones() as usize;
        for &(i, b) in &self.table[target][used] {
            if current & b == 0 {
                let next = current | b;
                if self.holes[target].iter().any(|&(u, v)| next & u == v) {
                    continue;
                }
                pieces[i] = b;
                self.backtrack(next, used | (1 << i), pieces, store);
            }
        }
    }
}

struct Checker {
    edges: [Bitboard; 4],
    unit_x: (Bitboard, Bitboard),
    unit_y: (Bitboard, Bitboard),
}

impl Checker {
    fn new(rows: usize, cols: usize) -> Self {
        let edge_x = (0..cols).map(|i| 1 << i).sum::<u64>();
        let edge_y = (0..rows).map(|i| 1 << (i * cols)).sum::<u64>();
        Self {
            edges: [
                edge_x,
                edge_y,
                edge_x << ((rows - 1) * cols),
                edge_y << (cols - 1),
            ],
            unit_x: (1 | (1 << 1), 1 | (1 << 2)),
            unit_y: (1 | (1 << cols), 1 | (1 << (cols * 2))),
        }
    }
    fn check_corner_space(&self, u: Bitboard) -> bool {
        [0, 1, 2, 3, 0].windows(2).any(|w| {
            let (e0, e1) = (self.edges[w[0]], self.edges[w[1]]);
            (e0 & u) != 0 && (e1 & u) != 0 && (e0 & e1) & u == 0
        })
    }
    fn check_hole(&self, u: Bitboard) -> bool {
        [self.unit_x, self.unit_y, self.unit_x, self.unit_y]
            .iter()
            .zip(&self.edges)
            .any(|(units, edge)| {
                let masked = u & edge;
                masked != 0 && masked % units.0 != 0 && masked % units.1 == 0
            })
    }
}
