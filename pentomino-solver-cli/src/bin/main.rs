use clap::{Parser, ValueEnum};
use pentomino_solver::solvers::{SimpleSolver, Solver};
use std::time::Instant;

/// Pentomino solver
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Show verbose output
    #[arg(short, long)]
    verbose: bool,
    /// Show all solutions
    #[arg(short, long)]
    show: bool,
    #[arg(short, long, value_enum, default_value_t = Board::Rect6x10)]
    board: Board,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum Board {
    Rect3x20,
    Rect4x15,
    Rect5x12,
    Rect6x10,
    Rect8x8_2x2,
}

fn main() {
    let args = Args::parse();

    let (solver, initial) = match args.board {
        Board::Rect3x20 => (SimpleSolver::new(3, 20), 0.into()),
        Board::Rect4x15 => (SimpleSolver::new(4, 15), 0.into()),
        Board::Rect5x12 => (SimpleSolver::new(5, 12), 0.into()),
        Board::Rect6x10 => (SimpleSolver::new(6, 10), 0.into()),
        Board::Rect8x8_2x2 => (
            SimpleSolver::new(8, 8),
            [27, 28, 35, 36].iter().map(|&p| 1 << p).sum::<u64>().into(),
        ),
    };
    let (solutions, elapsed) = {
        let now = Instant::now();
        let solutions = solver.solve(initial);
        let elapsed = now.elapsed();
        (solutions, elapsed)
    };
    if args.show {
        for solution in &solutions {
            if let Some(result) = solver.represent_solution(solution) {
                for row in result {
                    let mut line = String::new();
                    for col in row {
                        line += &(match col {
                            Some(p) => p.to_string(),
                            None => String::from(" "),
                        });
                    }
                    println!("{line}");
                }
                println!();
            }
        }
    }
    if args.verbose {
        println!("Found {} solutions in {elapsed:?}", solutions.len());
    } else {
        println!("Found {} solutions", solutions.len());
    }
}
