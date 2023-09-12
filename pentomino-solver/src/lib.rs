pub mod solvers;

pub const NUM_PIECES: usize = 12;

pub trait Solver {
    fn new(rows: usize, cols: usize) -> Self;
    fn solve(&self, start: u64) -> Vec<Vec<u64>>;
    fn show_solution(&self, solution: &[u64]) -> Vec<String>;
}
