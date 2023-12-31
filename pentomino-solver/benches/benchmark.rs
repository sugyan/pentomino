#![feature(test)]
extern crate test;
use pentomino_solver::solvers::OptimizedSolverType;
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
fn bench_8x8_2x2_optimized_small(b: &mut test::Bencher) {
    let solver = OptimizedSolver::new(8, 8, OptimizedSolverType::SmallTable);
    let initial = [27, 28, 35, 36].iter().map(|&p| 1 << p).sum::<u64>().into();
    b.iter(|| {
        let solutions = solver.solve(initial, false);
        assert_eq!(solutions.len(), 520);
    });
}

#[bench]
fn bench_8x8_2x2_optimized_large(b: &mut test::Bencher) {
    let solver = OptimizedSolver::new(8, 8, OptimizedSolverType::LargeTable);
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
fn bench_8x8_2x2_optimized_small_unique(b: &mut test::Bencher) {
    let solver = OptimizedSolver::new(8, 8, OptimizedSolverType::SmallTable);
    let initial = [27, 28, 35, 36].iter().map(|&p| 1 << p).sum::<u64>().into();
    b.iter(|| {
        let solutions = solver.solve(initial, true);
        assert_eq!(solutions.len(), 65);
    });
}

#[bench]
fn bench_8x8_2x2_optimized_large_unique(b: &mut test::Bencher) {
    let solver = OptimizedSolver::new(8, 8, OptimizedSolverType::LargeTable);
    let initial = [27, 28, 35, 36].iter().map(|&p| 1 << p).sum::<u64>().into();
    b.iter(|| {
        let solutions = solver.solve(initial, true);
        assert_eq!(solutions.len(), 65);
    });
}

#[bench]
fn bench_6x10_optimized_small(b: &mut test::Bencher) {
    let solver = OptimizedSolver::new(6, 10, OptimizedSolverType::SmallTable);
    let initial = Default::default();
    b.iter(|| {
        let solutions = solver.solve(initial, false);
        assert_eq!(solutions.len(), 9356);
    });
}

#[bench]
fn bench_6x10_optimized_large(b: &mut test::Bencher) {
    let solver = OptimizedSolver::new(6, 10, OptimizedSolverType::LargeTable);
    let initial = Default::default();
    b.iter(|| {
        let solutions = solver.solve(initial, false);
        assert_eq!(solutions.len(), 9356);
    });
}

#[bench]
fn bench_6x10_optimized_small_unique(b: &mut test::Bencher) {
    let solver = OptimizedSolver::new(6, 10, OptimizedSolverType::SmallTable);
    let initial = Default::default();
    b.iter(|| {
        let solutions = solver.solve(initial, true);
        assert_eq!(solutions.len(), 2339);
    });
}

#[bench]
fn bench_6x10_optimized_large_unique(b: &mut test::Bencher) {
    let solver = OptimizedSolver::new(6, 10, OptimizedSolverType::LargeTable);
    let initial = Default::default();
    b.iter(|| {
        let solutions = solver.solve(initial, true);
        assert_eq!(solutions.len(), 2339);
    });
}

#[bench]
fn bench_5x12_optimized_small(b: &mut test::Bencher) {
    let solver = OptimizedSolver::new(5, 12, OptimizedSolverType::SmallTable);
    let initial = Default::default();
    b.iter(|| {
        let solutions = solver.solve(initial, false);
        assert_eq!(solutions.len(), 4040);
    });
}

#[bench]
fn bench_5x12_optimized_large(b: &mut test::Bencher) {
    let solver = OptimizedSolver::new(5, 12, OptimizedSolverType::LargeTable);
    let initial = Default::default();
    b.iter(|| {
        let solutions = solver.solve(initial, false);
        assert_eq!(solutions.len(), 4040);
    });
}

#[bench]
fn bench_5x12_optimized_small_unique(b: &mut test::Bencher) {
    let solver = OptimizedSolver::new(5, 12, OptimizedSolverType::SmallTable);
    let initial = Default::default();
    b.iter(|| {
        let solutions = solver.solve(initial, true);
        assert_eq!(solutions.len(), 1010);
    });
}

#[bench]
fn bench_5x12_optimized_large_unique(b: &mut test::Bencher) {
    let solver = OptimizedSolver::new(5, 12, OptimizedSolverType::LargeTable);
    let initial = Default::default();
    b.iter(|| {
        let solutions = solver.solve(initial, true);
        assert_eq!(solutions.len(), 1010);
    });
}

#[bench]
fn bench_4x15_optimized_small(b: &mut test::Bencher) {
    let solver = OptimizedSolver::new(4, 15, OptimizedSolverType::SmallTable);
    let initial = Default::default();
    b.iter(|| {
        let solutions = solver.solve(initial, false);
        assert_eq!(solutions.len(), 1472);
    });
}

#[bench]
fn bench_4x15_optimized_large(b: &mut test::Bencher) {
    let solver = OptimizedSolver::new(4, 15, OptimizedSolverType::LargeTable);
    let initial = Default::default();
    b.iter(|| {
        let solutions = solver.solve(initial, false);
        assert_eq!(solutions.len(), 1472);
    });
}

#[bench]
fn bench_4x15_optimized_small_unique(b: &mut test::Bencher) {
    let solver = OptimizedSolver::new(4, 15, OptimizedSolverType::SmallTable);
    let initial = Default::default();
    b.iter(|| {
        let solutions = solver.solve(initial, true);
        assert_eq!(solutions.len(), 368);
    });
}

#[bench]
fn bench_4x15_optimized_large_unique(b: &mut test::Bencher) {
    let solver = OptimizedSolver::new(4, 15, OptimizedSolverType::LargeTable);
    let initial = Default::default();
    b.iter(|| {
        let solutions = solver.solve(initial, true);
        assert_eq!(solutions.len(), 368);
    });
}

#[bench]
fn bench_3x20_optimized_small(b: &mut test::Bencher) {
    let solver = OptimizedSolver::new(3, 20, OptimizedSolverType::SmallTable);
    let initial = Default::default();
    b.iter(|| {
        let solutions = solver.solve(initial, false);
        assert_eq!(solutions.len(), 8);
    });
}

#[bench]
fn bench_3x20_optimized_large(b: &mut test::Bencher) {
    let solver = OptimizedSolver::new(3, 20, OptimizedSolverType::LargeTable);
    let initial = Default::default();
    b.iter(|| {
        let solutions = solver.solve(initial, false);
        assert_eq!(solutions.len(), 8);
    });
}

#[bench]
fn bench_3x20_optimized_small_unique(b: &mut test::Bencher) {
    let solver = OptimizedSolver::new(3, 20, OptimizedSolverType::SmallTable);
    let initial = Default::default();
    b.iter(|| {
        let solutions = solver.solve(initial, true);
        assert_eq!(solutions.len(), 2);
    });
}

#[bench]
fn bench_3x20_optimized_large_unique(b: &mut test::Bencher) {
    let solver = OptimizedSolver::new(3, 20, OptimizedSolverType::LargeTable);
    let initial = Default::default();
    b.iter(|| {
        let solutions = solver.solve(initial, true);
        assert_eq!(solutions.len(), 2);
    });
}
