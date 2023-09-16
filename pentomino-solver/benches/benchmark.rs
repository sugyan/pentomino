#![feature(test)]
extern crate test;
use pentomino_solver::solvers::{SimpleSolver, Solver};

#[bench]
fn bench_8x8_2x2(b: &mut test::Bencher) {
    let solver = SimpleSolver::new(8, 8);
    let initial = [27, 28, 35, 36].iter().map(|&p| 1 << p).sum::<u64>().into();
    b.iter(|| {
        let solutions = solver.solve(initial, false);
        assert_eq!(solutions.len(), 520);
    });
}
