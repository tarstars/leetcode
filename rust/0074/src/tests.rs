use super::*;

/// Looks at every cell. Trivially correct, and shares nothing with a binary
/// search — which is the point of having it.
fn reference(matrix: &[Vec<i32>], target: i32) -> bool {
    matrix.iter().flatten().any(|&value| value == target)
}

fn search(matrix: &[Vec<i32>], target: i32) -> bool {
    Solution::search_matrix(matrix.to_vec(), target)
}

fn check(matrix: &[Vec<i32>], target: i32) {
    assert_eq!(
        search(matrix, target),
        reference(matrix, target),
        "target {target} in {matrix:?}"
    );
}

/// Confirms a generated matrix really satisfies the statement's two properties,
/// so a failure is never blamed on an illegal input.
fn assert_well_formed(matrix: &[Vec<i32>]) {
    let width = matrix[0].len();

    for (r, row) in matrix.iter().enumerate() {
        assert_eq!(row.len(), width, "row {r} has a different width");
        assert!(
            row.windows(2).all(|w| w[0] <= w[1]),
            "row {r} is not sorted: {row:?}"
        );
        if r > 0 {
            let previous_last = *matrix[r - 1].last().expect("non-empty row");
            assert!(
                row[0] > previous_last,
                "row {r} starts at {} which is not above {previous_last}",
                row[0]
            );
        }
    }
}

/// Deterministic matrix generation, so any failure reproduces.
struct Rng(u64);

impl Rng {
    fn next_value(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn below(&mut self, bound: u64) -> i64 {
        (self.next_value() % bound) as i64
    }

    /// Walks upward from `start`, stepping by 0..=`spread` within a row and by
    /// at least 1 across a row boundary — exactly the statement's two rules.
    /// A `spread` of 0 produces runs of equal values inside each row.
    fn matrix(&mut self, rows: usize, columns: usize, start: i64, spread: u64) -> Vec<Vec<i32>> {
        let mut value = start;
        let mut out = Vec::with_capacity(rows);

        for r in 0..rows {
            let mut row = Vec::with_capacity(columns);
            for c in 0..columns {
                if r > 0 || c > 0 {
                    let step = self.below(spread + 1);
                    value += if c == 0 { step.max(1) } else { step };
                }
                row.push(value as i32);
            }
            out.push(row);
        }

        out
    }
}

fn example_matrix() -> Vec<Vec<i32>> {
    vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]]
}

#[test]
fn example_1() {
    assert!(search(&example_matrix(), 3));
}

/// 13 falls in the gap between rows, which is where a row-selection off-by-one
/// tends to report a false positive.
#[test]
fn example_2() {
    assert!(!search(&example_matrix(), 13));
}

/// Every value that is present must be found, wherever it sits.
#[test]
fn every_value_in_the_matrix_is_found() {
    let matrix = example_matrix();

    for row in &matrix {
        for &value in row {
            assert!(search(&matrix, value), "{value} should be found");
        }
    }
}

/// Every value that is absent must be rejected, including the ones just outside
/// the matrix's range and the ones falling between rows.
#[test]
fn absent_values_are_rejected() {
    let matrix = example_matrix();

    for target in -20..=80 {
        check(&matrix, target);
    }
}

/// A single cell, hit and missed.
#[test]
fn a_single_cell() {
    assert!(search(&[vec![5]], 5));
    assert!(!search(&[vec![5]], 4));
    assert!(!search(&[vec![5]], 6));
    assert!(search(&[vec![-10_000]], -10_000));
    assert!(search(&[vec![10_000]], 10_000));
}

/// One row, and one column — both degenerate shapes for the index arithmetic.
#[test]
fn a_single_row_or_column() {
    let row = vec![vec![1, 3, 5, 7, 9]];
    for target in 0..=10 {
        check(&row, target);
    }

    let column = vec![vec![1], vec![3], vec![5], vec![7], vec![9]];
    for target in 0..=10 {
        check(&column, target);
    }
}

/// Rows may repeat a value, since they are only non-decreasing.
#[test]
fn repeated_values_within_a_row() {
    let matrix = vec![vec![1, 1, 1, 2], vec![5, 5, 9, 9], vec![10, 10, 10, 10]];
    assert_well_formed(&matrix);

    for target in -2..=13 {
        check(&matrix, target);
    }
}

/// Negative values, and the extremes of the permitted range.
#[test]
fn negative_and_extreme_values() {
    let matrix = vec![
        vec![-10_000, -5_000, -1],
        vec![0, 1, 2],
        vec![9_998, 9_999, 10_000],
    ];
    assert_well_formed(&matrix);

    for target in [
        -10_001, -10_000, -5_000, -2, -1, 0, 2, 3, 9_997, 10_000, 10_001,
    ] {
        check(&matrix, target);
    }
}

/// Generated matrices across many shapes, each probed at every value it holds,
/// at the values either side of each, and outside its range entirely.
#[test]
fn matches_reference_on_generated_matrices() {
    let mut rng = Rng(0x243F_6A88_85A3_08D3);

    for rows in 1..=6 {
        for columns in 1..=6 {
            for spread in [0u64, 1, 3, 10] {
                let matrix = rng.matrix(rows, columns, -50, spread);
                assert_well_formed(&matrix);

                let values: Vec<i32> = matrix.iter().flatten().copied().collect();
                let low = values[0];
                let high = values[values.len() - 1];

                for &value in &values {
                    check(&matrix, value);
                    check(&matrix, value - 1);
                    check(&matrix, value + 1);
                }
                check(&matrix, low - 1);
                check(&matrix, high + 1);
            }
        }
    }
}

/// Contiguous values leave no gaps, so every probe inside the range must hit.
#[test]
fn a_matrix_of_consecutive_values() {
    for rows in 1..=5usize {
        for columns in 1..=5usize {
            let matrix: Vec<Vec<i32>> = (0..rows)
                .map(|r| (0..columns).map(|c| (r * columns + c) as i32).collect())
                .collect();
            assert_well_formed(&matrix);

            for target in -1..=(rows * columns) as i32 {
                check(&matrix, target);
            }
        }
    }
}

/// 100 x 100 is the constraint's maximum — 10,000 cells, which the statement
/// requires be searched in O(log(m * n)).
#[test]
fn largest_allowed_matrix() {
    let mut rng = Rng(0xDEAD_BEEF_CAFE_F00D);
    let matrix = rng.matrix(100, 100, -10_000, 1);
    assert_well_formed(&matrix);

    let values: Vec<i32> = matrix.iter().flatten().copied().collect();
    for &value in &values {
        assert!(search(&matrix, value), "{value} should be found");
    }
    for probe in [-10_001, values[0] - 1, values[values.len() - 1] + 1] {
        check(&matrix, probe);
    }

    // Every value from -10000 upward, present or not.
    for target in -10_000..=-9_000 {
        check(&matrix, target);
    }
}
