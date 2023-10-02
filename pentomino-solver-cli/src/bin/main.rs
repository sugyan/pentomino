use clap::{Parser, ValueEnum};
use colored::*;
use pentomino_solver::solvers::OptimizedSolverType;
use pentomino_solver::solvers::{DefaultSolver, OptimizedSolver, SimpleSolver};
use pentomino_solver::Piece;
use pentomino_solver::Solver as PentominoSolver;
use std::time::Instant;
use supports_color::Stream;

/// Pentomino solver
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Color mode
    #[arg(short, long)]
    color: bool,
    /// Quiet mode
    #[arg(short, long)]
    quiet: bool,
    /// Unique mode (Discard solutions that are rotations or reflections of others)
    #[arg(short, long)]
    unique: bool,
    /// Board type
    #[arg(short, long, value_enum, default_value_t = Board::Rect6x10)]
    board: Board,
    /// Solver type
    #[arg(short, long, value_enum, default_value_t = Solver::Default)]
    solver: Solver,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum Board {
    Rect3x20,
    Rect4x15,
    Rect5x12,
    Rect6x10,
    Rect8x8_2x2,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum Solver {
    Simple,
    Default,
    OptimizedSmall,
    OptimizedLarge,
}

impl Solver {
    fn create_solver(&self, rows: usize, cols: usize) -> Box<dyn PentominoSolver> {
        match self {
            Solver::Simple => Box::new(SimpleSolver::new(rows, cols)),
            Solver::Default => Box::new(DefaultSolver::new(rows, cols)),
            Solver::OptimizedSmall => Box::new(OptimizedSolver::new(
                rows,
                cols,
                OptimizedSolverType::SmallTable,
            )),
            Solver::OptimizedLarge => Box::new(OptimizedSolver::new(
                rows,
                cols,
                OptimizedSolverType::LargeTable,
            )),
        }
    }
}

fn output(piece: &Option<Piece>, color: bool) -> String {
    if color {
        if let Some(support) = supports_color::on(Stream::Stdout) {
            if support.has_16m {
                return match piece {
                    Some(Piece::O) => "  ".on_truecolor(255, 128, 128).to_string(),
                    Some(Piece::P) => "  ".on_truecolor(255, 255, 128).to_string(),
                    Some(Piece::Q) => "  ".on_truecolor(128, 255, 128).to_string(),
                    Some(Piece::R) => "  ".on_truecolor(128, 255, 255).to_string(),
                    Some(Piece::S) => "  ".on_truecolor(128, 128, 255).to_string(),
                    Some(Piece::T) => "  ".on_truecolor(255, 128, 255).to_string(),
                    Some(Piece::U) => "  ".on_truecolor(128, 0, 0).to_string(),
                    Some(Piece::V) => "  ".on_truecolor(128, 128, 0).to_string(),
                    Some(Piece::W) => "  ".on_truecolor(0, 128, 0).to_string(),
                    Some(Piece::X) => "  ".on_truecolor(0, 128, 128).to_string(),
                    Some(Piece::Y) => "  ".on_truecolor(0, 0, 128).to_string(),
                    Some(Piece::Z) => "  ".on_truecolor(128, 0, 128).to_string(),
                    None => String::from("  "),
                };
            } else {
                // TODO
            }
        }
    }
    match piece {
        Some(p) => p.to_string(),
        None => String::from(" "),
    }
}

fn main() {
    let args = Args::parse();

    let ((rows, cols), initial) = match args.board {
        Board::Rect3x20 => ((3, 20), 0),
        Board::Rect4x15 => ((4, 15), 0),
        Board::Rect5x12 => ((5, 12), 0),
        Board::Rect6x10 => ((6, 10), 0),
        Board::Rect8x8_2x2 => (
            (8, 8),
            [27, 28, 35, 36].iter().map(|&p| 1 << p).sum::<u64>(),
        ),
    };
    let solver = args.solver.create_solver(rows, cols);
    let (solutions, elapsed) = {
        let now = Instant::now();
        let solutions = solver.solve(initial, args.unique);
        let elapsed = now.elapsed();
        (solutions, elapsed)
    };
    if !args.quiet {
        for solution in &solutions {
            for row in solver.represent_solution(solution) {
                let mut line = String::new();
                for col in &row {
                    line += &output(col, args.color);
                }
                println!("{line}");
            }
            println!();
        }
    }
    println!("Found {} solutions in {elapsed:?}", solutions.len());
}
