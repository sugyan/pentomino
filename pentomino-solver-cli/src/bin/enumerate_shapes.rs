const T: bool = true;
const F: bool = false;
const BLOCKS: [[[bool; 5]; 5]; 12] = [
    // O (I)
    [
        [T, F, F, F, F],
        [T, F, F, F, F],
        [T, F, F, F, F],
        [T, F, F, F, F],
        [T, F, F, F, F],
    ],
    // P
    [
        [T, T, F, F, F],
        [T, T, F, F, F],
        [T, F, F, F, F],
        [F, F, F, F, F],
        [F, F, F, F, F],
    ],
    // Q (L)
    [
        [T, T, F, F, F],
        [F, T, F, F, F],
        [F, T, F, F, F],
        [F, T, F, F, F],
        [F, F, F, F, F],
    ],
    // R (F)
    [
        [F, T, T, F, F],
        [T, T, F, F, F],
        [F, T, F, F, F],
        [F, F, F, F, F],
        [F, F, F, F, F],
    ],
    // S (N)
    [
        [F, F, T, T, F],
        [T, T, T, F, F],
        [F, F, F, F, F],
        [F, F, F, F, F],
        [F, F, F, F, F],
    ],
    // T
    [
        [T, T, T, F, F],
        [F, T, F, F, F],
        [F, T, F, F, F],
        [F, F, F, F, F],
        [F, F, F, F, F],
    ],
    // U
    [
        [T, F, T, F, F],
        [T, T, T, F, F],
        [F, F, F, F, F],
        [F, F, F, F, F],
        [F, F, F, F, F],
    ],
    // V
    [
        [T, F, F, F, F],
        [T, F, F, F, F],
        [T, T, T, F, F],
        [F, F, F, F, F],
        [F, F, F, F, F],
    ],
    // W
    [
        [T, F, F, F, F],
        [T, T, F, F, F],
        [F, T, T, F, F],
        [F, F, F, F, F],
        [F, F, F, F, F],
    ],
    // X
    [
        [F, T, F, F, F],
        [T, T, T, F, F],
        [F, T, F, F, F],
        [F, F, F, F, F],
        [F, F, F, F, F],
    ],
    // Y
    [
        [F, F, T, F, F],
        [T, T, T, T, F],
        [F, F, F, F, F],
        [F, F, F, F, F],
        [F, F, F, F, F],
    ],
    // Z
    [
        [T, T, F, F, F],
        [F, T, F, F, F],
        [F, T, T, F, F],
        [F, F, F, F, F],
        [F, F, F, F, F],
    ],
];

fn mat_rot90(block: &[[bool; 5]; 5]) -> [[bool; 5]; 5] {
    let mut new = [[false; 5]; 5];
    for (y, row) in block.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            new[x][4 - y] = *col;
        }
    }
    new
}

fn mat_flip(block: &[[bool; 5]; 5]) -> [[bool; 5]; 5] {
    let mut new = [[false; 5]; 5];
    for (y, row) in block.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            new[y][4 - x] = *col;
        }
    }
    new
}

fn transform(block: &[[bool; 5]; 5], flip: bool, rot90: usize) -> [[bool; 5]; 5] {
    let mut new = *block;
    if flip {
        new = mat_flip(&new);
    }
    for _ in 0..(rot90 % 4) {
        new = mat_rot90(&new);
    }
    new
}

fn enumerate_shapes(block: &[[bool; 5]; 5]) -> Vec<[[bool; 5]; 5]> {
    let mut shapes = Vec::new();
    for &flip in &[false, true] {
        for rot90 in 0..4 {
            shapes.push(transform(block, flip, rot90));
        }
    }
    shapes
}

fn main() {
    assert!(BLOCKS
        .iter()
        .all(|b| b.iter().flatten().filter(|&b| *b).count() == 5));
    for block in BLOCKS {
        let mut shapes = Vec::new();
        for shape in enumerate_shapes(&block) {
            let mut v = Vec::with_capacity(5);
            for (y, row) in shape.iter().enumerate() {
                for (x, col) in row.iter().enumerate() {
                    if *col {
                        v.push((x, y));
                    }
                }
            }
            let xmin = *v.iter().map(|(x, _)| x).min().unwrap();
            let ymin = *v.iter().map(|(_, y)| y).min().unwrap();
            v.iter_mut().for_each(|(x, y)| {
                *x -= xmin;
                *y -= ymin;
            });
            if !shapes.contains(&v) {
                shapes.push(v);
            }
        }
        println!("vec![");
        for shape in &shapes {
            println!("  {:?},", shape);
        }
        println!("],");
    }
}
