use super::*;

/// For every pair of rows, mark the columns that are solid `'1'` across the
/// whole band, then take the longest run of them. O(rows^2 * cols), which is
/// fast enough even at 200 x 200, and it shares nothing with the row-by-row
/// histogram approach.
fn reference(matrix: &[Vec<char>]) -> i32 {
    let rows = matrix.len();
    if rows == 0 || matrix[0].is_empty() {
        return 0;
    }

    let mut best = 0;
    for top in 0..rows {
        let mut solid = vec![true; matrix[0].len()];
        for (bottom, row) in matrix.iter().enumerate().skip(top) {
            let height = (bottom - top + 1) as i32;
            let mut run = 0;
            for (is_solid, &cell) in solid.iter_mut().zip(row) {
                *is_solid &= cell == '1';
                if *is_solid {
                    run += 1;
                    best = best.max(run * height);
                } else {
                    run = 0;
                }
            }
        }
    }
    best
}

/// `["101", "010"]` is far easier to read than nested `vec!['1', '0', '1']`.
fn grid(rows: &[&str]) -> Vec<Vec<char>> {
    rows.iter().map(|row| row.chars().collect()).collect()
}

fn area(rows: &[&str]) -> i32 {
    Solution::maximal_rectangle(grid(rows))
}

fn check(matrix: &[Vec<char>]) {
    let got = Solution::maximal_rectangle(matrix.to_vec());
    let want = reference(matrix);
    if got != want {
        // Only rendered on failure — the exhaustive sweeps call this 75,000
        // times and would otherwise spend their time allocating strings.
        let shown: Vec<String> = matrix.iter().map(|r| r.iter().collect()).collect();
        panic!("for {shown:?}: got {got}, want {want}");
    }
}

/// Every cell of an `rows x cols` grid read off the bits of `pattern`.
fn from_bits(rows: usize, cols: usize, pattern: u32) -> Vec<Vec<char>> {
    (0..rows)
        .map(|i| {
            (0..cols)
                .map(|j| {
                    if pattern >> (i * cols + j) & 1 == 1 {
                        '1'
                    } else {
                        '0'
                    }
                })
                .collect()
        })
        .collect()
}

/// Deterministic generation, so any failure reproduces exactly.
struct Rng(u64);

impl Rng {
    fn next_value(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    /// `ones_in_16` controls density: 0 is all zeros, 16 is all ones.
    fn matrix(&mut self, rows: usize, cols: usize, ones_in_16: u64) -> Vec<Vec<char>> {
        (0..rows)
            .map(|_| {
                (0..cols)
                    .map(|_| {
                        if self.next_value() % 16 < ones_in_16 {
                            '1'
                        } else {
                            '0'
                        }
                    })
                    .collect()
            })
            .collect()
    }
}

#[test]
fn example_1() {
    assert_eq!(area(&["10100", "10111", "11111", "10010"]), 6);
}

#[test]
fn example_2() {
    assert_eq!(area(&["0"]), 0);
}

#[test]
fn example_3() {
    assert_eq!(area(&["1"]), 1);
}

/// The two degenerate fills.
#[test]
fn uniform_matrices() {
    assert_eq!(area(&["111", "111", "111"]), 9);
    assert_eq!(area(&["000", "000", "000"]), 0);
    assert_eq!(area(&["1111"]), 4);
    assert_eq!(area(&["1", "1", "1", "1"]), 4);
}

/// One row is exactly problem 84 with every bar of height 0 or 1, so the answer
/// is the longest run of ones.
#[test]
fn a_single_row() {
    assert_eq!(area(&["10110"]), 2);
    assert_eq!(area(&["01110"]), 3);
    assert_eq!(area(&["00000"]), 0);
    assert_eq!(area(&["10101"]), 1);
}

/// A single column is the transpose of the same thing.
#[test]
fn a_single_column() {
    assert_eq!(area(&["1", "0", "1", "1", "0"]), 2);
    assert_eq!(area(&["0", "1", "1", "1", "0"]), 3);
    assert_eq!(area(&["0", "0"]), 0);
}

/// A tall-and-thin and a short-and-wide block in the same matrix: the winner is
/// decided by area, not by either dimension alone.
#[test]
fn tall_versus_wide() {
    assert_eq!(area(&["1000111", "1000111", "1000000", "1000000"]), 6);
    assert_eq!(area(&["1111", "0000", "1100", "1100"]), 4);
}

/// The best rectangle's bottom edge is not on the last row, so a solution that
/// only reports the final row's histogram fails here.
#[test]
fn the_winner_is_not_on_the_last_row() {
    assert_eq!(area(&["1111", "1111", "0010"]), 8);
    assert_eq!(area(&["111", "111", "111", "000"]), 9);
}

/// A zero in the middle of a column resets that column's run — the heights
/// carried between rows must not survive it.
#[test]
fn zeros_reset_a_column() {
    assert_eq!(area(&["111", "101", "111"]), 3);
    assert_eq!(area(&["11", "00", "11", "11"]), 4);
    check(&grid(&["1111", "1011", "1111", "1101"]));
}

/// Checkerboards have no rectangle bigger than a single cell.
#[test]
fn checkerboards() {
    assert_eq!(area(&["1010", "0101", "1010", "0101"]), 1);
    assert_eq!(area(&["0101", "1010", "0101"]), 1);
}

/// Staircases, where every row shifts the block by one column.
#[test]
fn staircases() {
    check(&grid(&["1000", "1100", "1110", "1111"]));
    check(&grid(&["1111", "1110", "1100", "1000"]));
    check(&grid(&["0011", "0110", "1100", "1000"]));
}

/// Every binary matrix whose shape is at most 4 x 4 and whose cell count is at
/// most 12 — a little under ten thousand grids. Any bug a small case can
/// express is somewhere in here.
#[test]
fn every_small_matrix() {
    let mut checked = 0u32;
    for rows in 1..=4usize {
        for cols in 1..=4usize {
            if rows * cols > 12 {
                continue;
            }
            for pattern in 0..1u32 << (rows * cols) {
                check(&from_bits(rows, cols, pattern));
                checked += 1;
            }
        }
    }
    assert_eq!(checked, 9_418, "the sweep changed size");
}

/// Every 4 x 4 matrix — 65,536 of them, the largest square the sweep can cover
/// exhaustively.
#[test]
fn every_four_by_four_matrix() {
    for pattern in 0..1u32 << 16 {
        check(&from_bits(4, 4, pattern));
    }
}

/// Random matrices across shapes and densities. Sparse ones exercise the reset
/// path, dense ones exercise long runs.
#[test]
fn matches_reference_on_random_matrices() {
    let mut rng = Rng(0x243F_6A88_85A3_08D3);

    for (rows, cols) in [(1, 1), (1, 20), (20, 1), (5, 7), (12, 12), (30, 18)] {
        for density in [1u64, 4, 8, 12, 15] {
            for _ in 0..8 {
                check(&rng.matrix(rows, cols, density));
            }
        }
    }
}

/// Transposing a matrix mirrors every rectangle, so the answer cannot change.
/// This catches a solution that treats rows and columns asymmetrically.
#[test]
fn the_answer_survives_transposition() {
    let mut rng = Rng(0x1357_9BDF_2468_ACE0);

    for _ in 0..150 {
        let matrix = rng.matrix(9, 13, 10);
        let transposed: Vec<Vec<char>> = (0..13)
            .map(|c| (0..9).map(|r| matrix[r][c]).collect())
            .collect();

        let upright = Solution::maximal_rectangle(matrix.clone());
        let sideways = Solution::maximal_rectangle(transposed);
        if upright != sideways {
            let shown: Vec<String> = matrix.iter().map(|r| r.iter().collect()).collect();
            panic!("{shown:?} gives {upright} but its transpose gives {sideways}");
        }
    }
}

/// The constraint maximum, 200 x 200, in the two shapes with an obvious answer.
#[test]
fn the_largest_possible_matrix() {
    assert_eq!(Solution::maximal_rectangle(vec![vec!['1'; 200]; 200]), 40_000);
    assert_eq!(Solution::maximal_rectangle(vec![vec!['0'; 200]; 200]), 0);
}

/// A solid block of known size embedded in an otherwise empty 200 x 200 grid,
/// plus a full row and a full column that must lose to it.
#[test]
fn a_planted_block() {
    let mut matrix = vec![vec!['0'; 200]; 200];

    // A full row of 200 and a full column of 200, each worth 200.
    matrix[7] = vec!['1'; 200];
    for row in matrix.iter_mut() {
        row[3] = '1';
    }
    // A 40 x 60 block worth 2400, which should win.
    for row in matrix.iter_mut().skip(100).take(40) {
        for cell in row.iter_mut().skip(120).take(60) {
            *cell = '1';
        }
    }

    assert_eq!(Solution::maximal_rectangle(matrix), 2_400);
}

/// Full size against the quadratic-in-rows oracle, at a density where large
/// rectangles are plentiful but not guaranteed.
#[test]
fn a_large_random_matrix() {
    let mut rng = Rng(0xDEAD_BEEF_CAFE_F00D);

    for density in [5u64, 11, 14] {
        check(&rng.matrix(200, 200, density));
    }
}

/// Alternating full rows: every rectangle is one row tall, so the answer is the
/// width. A solution that lets heights leak past a zero row reports more.
#[test]
fn alternating_full_rows() {
    let matrix: Vec<Vec<char>> = (0..200)
        .map(|r| vec![if r % 2 == 0 { '1' } else { '0' }; 200])
        .collect();

    assert_eq!(Solution::maximal_rectangle(matrix), 200);
}
