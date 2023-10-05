use js_sys::Array;
use pentomino_solver::solvers::{OptimizedSolver, OptimizedSolverType};
use pentomino_solver::{Bitboard, Solver, NUM_PIECES};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Solutions(Vec<[Bitboard; NUM_PIECES]>);

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
    pub fn solve(&self, initial: Bitboard, unique: bool) -> Solutions {
        Solutions(self.inner.solve(initial, unique))
    }
    pub fn represent_solution(&self, solutions: Solutions) -> Vec<JsValue> {
        solutions
            .0
            .iter()
            .map(|s| self.solution2jsvalue(s))
            .collect()
    }
    fn solution2jsvalue(&self, solution: &[Bitboard; NUM_PIECES]) -> JsValue {
        self.inner
            .represent_solution(solution)
            .iter()
            .map(|row| {
                let mut s = String::new();
                for col in row {
                    s += &(match col {
                        None => String::from("."),
                        Some(p) => p.to_string(),
                    });
                }
                JsValue::from(s)
            })
            .collect::<Array>()
            .into()
    }
}
