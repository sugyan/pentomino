use crate::NUM_PIECES;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Matrix([[bool; 5]; 5]);

impl Matrix {
    fn flip(&self) -> Self {
        let mut ret = [[false; 5]; 5];
        for (y, row) in self.0.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                ret[y][4 - x] = *col;
            }
        }
        Self(ret)
    }
    fn rot90(&self) -> Self {
        let mut ret = [[false; 5]; 5];
        for (y, row) in self.0.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                ret[x][4 - y] = *col;
            }
        }
        Self(ret)
    }
    fn transform(&self, flip: bool, rot90: usize) -> Self {
        let mut ret = *self;
        if flip {
            ret = ret.flip();
        }
        for _ in 0..(rot90 % 4) {
            ret = ret.rot90();
        }
        ret
    }
    fn normalized_coordinates(&self) -> Vec<(usize, usize)> {
        let mut ret = Vec::new();
        for (y, row) in self.0.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                if *col {
                    ret.push((x, y));
                }
            }
        }
        let (xmin, ymin) = ret
            .iter()
            .fold((5, 5), |(xmin, ymin), &(x, y)| (xmin.min(x), ymin.min(y)));
        ret.iter_mut().for_each(|(x, y)| {
            *x -= xmin;
            *y -= ymin;
        });
        ret
    }
}

const T: bool = true;
const F: bool = false;
const BLOCKS: [Matrix; NUM_PIECES] = [
    // O (I)
    Matrix([
        [T, F, F, F, F],
        [T, F, F, F, F],
        [T, F, F, F, F],
        [T, F, F, F, F],
        [T, F, F, F, F],
    ]),
    // P
    Matrix([
        [T, T, F, F, F],
        [T, T, F, F, F],
        [T, F, F, F, F],
        [F, F, F, F, F],
        [F, F, F, F, F],
    ]),
    // Q (L)
    Matrix([
        [T, T, F, F, F],
        [F, T, F, F, F],
        [F, T, F, F, F],
        [F, T, F, F, F],
        [F, F, F, F, F],
    ]),
    // R (F)
    Matrix([
        [F, T, T, F, F],
        [T, T, F, F, F],
        [F, T, F, F, F],
        [F, F, F, F, F],
        [F, F, F, F, F],
    ]),
    // S (N)
    Matrix([
        [F, F, T, T, F],
        [T, T, T, F, F],
        [F, F, F, F, F],
        [F, F, F, F, F],
        [F, F, F, F, F],
    ]),
    // T
    Matrix([
        [T, T, T, F, F],
        [F, T, F, F, F],
        [F, T, F, F, F],
        [F, F, F, F, F],
        [F, F, F, F, F],
    ]),
    // U
    Matrix([
        [T, F, T, F, F],
        [T, T, T, F, F],
        [F, F, F, F, F],
        [F, F, F, F, F],
        [F, F, F, F, F],
    ]),
    // V
    Matrix([
        [T, F, F, F, F],
        [T, F, F, F, F],
        [T, T, T, F, F],
        [F, F, F, F, F],
        [F, F, F, F, F],
    ]),
    // W
    Matrix([
        [T, F, F, F, F],
        [T, T, F, F, F],
        [F, T, T, F, F],
        [F, F, F, F, F],
        [F, F, F, F, F],
    ]),
    // X
    Matrix([
        [F, T, F, F, F],
        [T, T, T, F, F],
        [F, T, F, F, F],
        [F, F, F, F, F],
        [F, F, F, F, F],
    ]),
    // Y
    Matrix([
        [F, F, T, F, F],
        [T, T, T, T, F],
        [F, F, F, F, F],
        [F, F, F, F, F],
        [F, F, F, F, F],
    ]),
    // Z
    Matrix([
        [T, T, F, F, F],
        [F, T, F, F, F],
        [F, T, T, F, F],
        [F, F, F, F, F],
        [F, F, F, F, F],
    ]),
];

pub(crate) fn calculate_shapes() -> Vec<Vec<Vec<(usize, usize)>>> {
    let mut ret = Vec::new();
    for block in BLOCKS {
        let mut shapes = Vec::new();
        for flip in [false, true] {
            for rot in 0..4 {
                let shape = block.transform(flip, rot).normalized_coordinates();
                if !shapes.contains(&shape) {
                    shapes.push(shape);
                }
            }
        }
        ret.push(shapes);
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_flip() {
        let m = Matrix([
            [T, T, T, F, F],
            [F, T, F, F, F],
            [F, T, F, F, F],
            [F, F, F, F, F],
            [F, F, F, F, F],
        ]);
        assert_eq!(
            m.flip(),
            Matrix([
                [F, F, T, T, T],
                [F, F, F, T, F],
                [F, F, F, T, F],
                [F, F, F, F, F],
                [F, F, F, F, F],
            ])
        );
        assert_eq!(m.flip().flip(), m);
    }

    #[test]
    fn matrix_rot90() {
        let m = Matrix([
            [T, T, T, F, F],
            [F, T, F, F, F],
            [F, T, F, F, F],
            [F, F, F, F, F],
            [F, F, F, F, F],
        ]);
        assert_eq!(
            m.rot90(),
            Matrix([
                [F, F, F, F, T],
                [F, F, T, T, T],
                [F, F, F, F, T],
                [F, F, F, F, F],
                [F, F, F, F, F],
            ])
        );
        assert_eq!(
            m.rot90().rot90(),
            Matrix([
                [F, F, F, F, F],
                [F, F, F, F, F],
                [F, F, F, T, F],
                [F, F, F, T, F],
                [F, F, T, T, T],
            ])
        );
        assert_eq!(
            m.rot90().rot90().rot90(),
            Matrix([
                [F, F, F, F, F],
                [F, F, F, F, F],
                [T, F, F, F, F],
                [T, T, T, F, F],
                [T, F, F, F, F],
            ])
        );
        assert_eq!(m.rot90().rot90().rot90().rot90(), m);
    }

    #[test]
    fn transform() {
        let m = Matrix([
            [T, T, T, F, F],
            [F, T, F, F, F],
            [F, T, F, F, F],
            [F, F, F, F, F],
            [F, F, F, F, F],
        ]);
        assert_eq!(
            m.transform(true, 1),
            Matrix([
                [F, F, F, F, F],
                [F, F, F, F, F],
                [F, F, F, F, T],
                [F, F, T, T, T],
                [F, F, F, F, T],
            ])
        );
        assert_eq!(
            m.transform(true, 3),
            Matrix([
                [T, F, F, F, F],
                [T, T, T, F, F],
                [T, F, F, F, F],
                [F, F, F, F, F],
                [F, F, F, F, F],
            ])
        );
    }

    #[test]
    fn normalized_coordinates() {
        assert_eq!(
            Matrix([
                [T, T, T, F, F],
                [F, T, F, F, F],
                [F, T, F, F, F],
                [F, F, F, F, F],
                [F, F, F, F, F],
            ])
            .normalized_coordinates(),
            vec![(0, 0), (1, 0), (2, 0), (1, 1), (1, 2)]
        );
        assert_eq!(
            Matrix([
                [F, F, T, T, T],
                [F, F, F, T, F],
                [F, F, F, T, F],
                [F, F, F, F, F],
                [F, F, F, F, F],
            ])
            .normalized_coordinates(),
            vec![(0, 0), (1, 0), (2, 0), (1, 1), (1, 2)]
        );
    }

    #[test]
    fn calculated_shapes() {
        assert_eq!(
            calculate_shapes()
                .iter()
                .map(|shapes| shapes.len())
                .sum::<usize>(),
            63
        );
    }
}
