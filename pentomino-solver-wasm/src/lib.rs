use pentomino_solver::solvers::{OptimizedSolver, OptimizedSolverType};
use pentomino_solver::{Bitboard, Solver};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmSolver {
    inner: OptimizedSolver,
}

#[wasm_bindgen]
impl WasmSolver {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            inner: OptimizedSolver::new(rows, cols, OptimizedSolverType::LargeTable),
        }
    }
    pub fn solve(&self, initial: Bitboard, unique: bool) -> Vec<Bitboard> {
        self.inner
            .solve(initial, unique)
            .into_iter()
            .flatten()
            .collect()
    }
}
