#![feature(test)]
extern crate test;
use pentomino_solver::solvers::{SimpleSolver, Solver};

#[bench]
fn bench_8x8_2x2(b: &mut test::Bencher) {
    let solver = SimpleSolver::new(8, 8);
    let start = [27, 28, 35, 36].iter().map(|&p| 1 << p).sum::<u64>();
    b.iter(|| {
        let solutions = solver.solve(start);
        assert_eq!(solutions.len(), 520);
    });
}
