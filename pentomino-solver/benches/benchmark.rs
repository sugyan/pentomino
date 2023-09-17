#![feature(test)]
extern crate test;
use pentomino_solver::solvers::{DefaultSolver, OptimizedSolver, SimpleSolver};
use pentomino_solver::Solver;

#[bench]
fn bench_8x8_2x2_simple(b: &mut test::Bencher) {
    let solver = SimpleSolver::new(8, 8);
    let initial = [27, 28, 35, 36].iter().map(|&p| 1 << p).sum::<u64>().into();
    b.iter(|| {
        let solutions = solver.solve(initial, false);
        assert_eq!(solutions.len(), 520);
    });
}

#[bench]
fn bench_8x8_2x2_default(b: &mut test::Bencher) {
    let solver = DefaultSolver::new(8, 8);
    let initial = [27, 28, 35, 36].iter().map(|&p| 1 << p).sum::<u64>().into();
    b.iter(|| {
        let solutions = solver.solve(initial, false);
        assert_eq!(solutions.len(), 520);
    });
}

#[bench]
fn bench_8x8_2x2_optimized(b: &mut test::Bencher) {
    let solver = OptimizedSolver::new(8, 8);
    let initial = [27, 28, 35, 36].iter().map(|&p| 1 << p).sum::<u64>().into();
    b.iter(|| {
        let solutions = solver.solve(initial, false);
        assert_eq!(solutions.len(), 520);
    });
}

#[bench]
fn bench_8x8_2x2_default_unique(b: &mut test::Bencher) {
    let solver = DefaultSolver::new(8, 8);
    let initial = [27, 28, 35, 36].iter().map(|&p| 1 << p).sum::<u64>().into();
    b.iter(|| {
        let solutions = solver.solve(initial, true);
        assert_eq!(solutions.len(), 65);
    });
}

#[bench]
fn bench_8x8_2x2_optimized_unique(b: &mut test::Bencher) {
    let solver = OptimizedSolver::new(8, 8);
    let initial = [27, 28, 35, 36].iter().map(|&p| 1 << p).sum::<u64>().into();
    b.iter(|| {
        let solutions = solver.solve(initial, true);
        assert_eq!(solutions.len(), 65);
    });
}
