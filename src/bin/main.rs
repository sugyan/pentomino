use std::array;
use std::time::Instant;

struct Solver {
    rows: usize,
    cols: usize,
    table: [[Vec<u64>; Self::NUM_PIECES]; 64],
}

impl Solver {
    const NUM_PIECES: usize = 12;

    fn new(rows: usize, cols: usize) -> Self {
        assert!(rows * cols <= 64);

        let shapes = [
            vec![
                [(0, 0), (0, 1), (0, 2), (0, 3), (0, 4)],
                [(0, 0), (1, 0), (2, 0), (3, 0), (4, 0)],
            ],
            vec![
                [(0, 0), (1, 0), (0, 1), (1, 1), (0, 2)],
                [(0, 0), (1, 0), (2, 0), (1, 1), (2, 1)],
                [(1, 0), (0, 1), (1, 1), (0, 2), (1, 2)],
                [(0, 0), (1, 0), (0, 1), (1, 1), (2, 1)],
                [(0, 0), (1, 0), (0, 1), (1, 1), (1, 2)],
                [(1, 0), (2, 0), (0, 1), (1, 1), (2, 1)],
                [(0, 0), (0, 1), (1, 1), (0, 2), (1, 2)],
                [(0, 0), (1, 0), (2, 0), (0, 1), (1, 1)],
            ],
            vec![
                [(0, 0), (1, 0), (1, 1), (1, 2), (1, 3)],
                [(3, 0), (0, 1), (1, 1), (2, 1), (3, 1)],
                [(0, 0), (0, 1), (0, 2), (0, 3), (1, 3)],
                [(0, 0), (1, 0), (2, 0), (3, 0), (0, 1)],
                [(0, 0), (1, 0), (0, 1), (0, 2), (0, 3)],
                [(0, 0), (1, 0), (2, 0), (3, 0), (3, 1)],
                [(1, 0), (1, 1), (1, 2), (0, 3), (1, 3)],
                [(0, 0), (0, 1), (1, 1), (2, 1), (3, 1)],
            ],
            vec![
                [(1, 0), (2, 0), (0, 1), (1, 1), (1, 2)],
                [(1, 0), (0, 1), (1, 1), (2, 1), (2, 2)],
                [(1, 0), (1, 1), (2, 1), (0, 2), (1, 2)],
                [(0, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
                [(0, 0), (1, 0), (1, 1), (2, 1), (1, 2)],
                [(2, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
                [(1, 0), (0, 1), (1, 1), (1, 2), (2, 2)],
                [(1, 0), (0, 1), (1, 1), (2, 1), (0, 2)],
            ],
            vec![
                [(2, 0), (3, 0), (0, 1), (1, 1), (2, 1)],
                [(0, 0), (0, 1), (0, 2), (1, 2), (1, 3)],
                [(1, 0), (2, 0), (3, 0), (0, 1), (1, 1)],
                [(0, 0), (0, 1), (1, 1), (1, 2), (1, 3)],
                [(0, 0), (1, 0), (1, 1), (2, 1), (3, 1)],
                [(1, 0), (0, 1), (1, 1), (0, 2), (0, 3)],
                [(0, 0), (1, 0), (2, 0), (2, 1), (3, 1)],
                [(1, 0), (1, 1), (0, 2), (1, 2), (0, 3)],
            ],
            vec![
                [(0, 0), (1, 0), (2, 0), (1, 1), (1, 2)],
                [(2, 0), (0, 1), (1, 1), (2, 1), (2, 2)],
                [(1, 0), (1, 1), (0, 2), (1, 2), (2, 2)],
                [(0, 0), (0, 1), (1, 1), (2, 1), (0, 2)],
            ],
            vec![
                [(0, 0), (2, 0), (0, 1), (1, 1), (2, 1)],
                [(0, 0), (1, 0), (0, 1), (0, 2), (1, 2)],
                [(0, 0), (1, 0), (2, 0), (0, 1), (2, 1)],
                [(0, 0), (1, 0), (1, 1), (0, 2), (1, 2)],
            ],
            vec![
                [(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)],
                [(0, 0), (1, 0), (2, 0), (0, 1), (0, 2)],
                [(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
                [(2, 0), (2, 1), (0, 2), (1, 2), (2, 2)],
            ],
            vec![
                [(0, 0), (0, 1), (1, 1), (1, 2), (2, 2)],
                [(1, 0), (2, 0), (0, 1), (1, 1), (0, 2)],
                [(0, 0), (1, 0), (1, 1), (2, 1), (2, 2)],
                [(2, 0), (1, 1), (2, 1), (0, 2), (1, 2)],
            ],
            vec![[(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)]],
            vec![
                [(2, 0), (0, 1), (1, 1), (2, 1), (3, 1)],
                [(0, 0), (0, 1), (0, 2), (1, 2), (0, 3)],
                [(0, 0), (1, 0), (2, 0), (3, 0), (1, 1)],
                [(1, 0), (0, 1), (1, 1), (1, 2), (1, 3)],
                [(1, 0), (0, 1), (1, 1), (2, 1), (3, 1)],
                [(0, 0), (0, 1), (1, 1), (0, 2), (0, 3)],
                [(0, 0), (1, 0), (2, 0), (3, 0), (2, 1)],
                [(1, 0), (1, 1), (0, 2), (1, 2), (1, 3)],
            ],
            vec![
                [(0, 0), (1, 0), (1, 1), (1, 2), (2, 2)],
                [(2, 0), (0, 1), (1, 1), (2, 1), (0, 2)],
                [(1, 0), (2, 0), (1, 1), (0, 2), (1, 2)],
                [(0, 0), (0, 1), (1, 1), (2, 1), (2, 2)],
            ],
        ];
        let mut table = array::from_fn(|_| array::from_fn(|_| Vec::new()));
        for (n, shape) in shapes.iter().enumerate() {
            for s in shape {
                let v = s.iter().map(|p| (1 << (p.0 + p.1 * cols))).sum::<u64>();
                let w = s.iter().map(|(x, _)| x).max().unwrap();
                let h = s.iter().map(|(_, y)| y).max().unwrap();
                for i in 0..cols - w {
                    for j in 0..rows - h {
                        let offset = i + j * cols;
                        table[s[0].0 + offset][n].push(v << offset);
                    }
                }
            }
        }
        Self { rows, cols, table }
    }
    fn solve(&self, start: u64) -> Vec<Vec<u64>> {
        fn backtrack(
            current: u64,
            remain: u32,
            table: &[[Vec<u64>; 12]; 64],
            path: &mut Vec<u64>,
            solutions: &mut Vec<Vec<u64>>,
        ) {
            if remain == 0 {
                solutions.push(path.clone());
                return;
            }
            let target = current.trailing_ones() as usize;
            for (i, candidate) in table[target].iter().enumerate() {
                if remain & (1 << i) != 0 {
                    for &c in candidate.iter() {
                        if current & c == 0 {
                            path.push(c);
                            backtrack(current | c, remain & !(1 << i), table, path, solutions);
                            path.pop();
                        }
                    }
                }
            }
        }

        let mut solutions = Vec::new();
        backtrack(
            start,
            (1 << Self::NUM_PIECES) - 1,
            &self.table,
            &mut Vec::with_capacity(Self::NUM_PIECES),
            &mut solutions,
        );
        solutions
    }
    fn show_solution(&self, solution: &[u64]) {
        for y in 0..self.rows {
            let mut row = String::new();
            for x in 0..self.cols {
                let z = 1 << (x + y * self.cols);
                if let Some(p) = solution.iter().find(|&p| p & z != 0) {
                    let i = self.table[p.trailing_zeros() as usize]
                        .iter()
                        .enumerate()
                        .find_map(|(i, v)| if v.contains(p) { Some(i) } else { None })
                        .unwrap();
                    row.push((b'O' + i as u8) as char);
                } else {
                    row.push(' ');
                }
            }
            println!("{row}");
        }
        println!();
    }
}

fn main() {
    let solver = Solver::new(8, 8);
    let start = [27, 28, 35, 36].iter().map(|&p| 1 << p).sum::<u64>();
    // let solver = Solver::new(6, 10);
    // let start = 0;
    {
        let now = Instant::now();
        let solutions = solver.solve(start);
        let elapsed = now.elapsed();
        for solution in &solutions {
            solver.show_solution(solution);
        }
        println!("Found {} solutions in {elapsed:?}", solutions.len());
    }
}
