use pentomino_solver::solvers::SimpleSolver;
use pentomino_solver::Solver;
use std::time::Instant;

fn main() {
    let solver = SimpleSolver::new(8, 8);
    let start = [27, 28, 35, 36].iter().map(|&p| 1 << p).sum::<u64>();
    // let solver = Solver::new(6, 10);
    // let start = 0;
    {
        let now = Instant::now();
        let solutions = solver.solve(start);
        let elapsed = now.elapsed();
        for solution in &solutions {
            for line in solver.show_solution(solution) {
                println!("{line}");
            }
            println!();
        }
        println!("Found {} solutions in {elapsed:?}", solutions.len());
    }
}
