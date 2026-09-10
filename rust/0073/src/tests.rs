use super::*;

/// Builds the answer out of place, from the *original* matrix: a cell survives
/// only if neither its row nor its column held a zero to begin with. Deciding
/// everything from the untouched input is what a sound in-place solution has to
/// simulate, and doing it with a fresh output makes the rule obvious.
fn reference(matrix: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let rows = matrix.len();
    let columns = matrix[0].len();

    let zero_row: Vec<bool> = (0..rows)
        .map(|r| (0..columns).any(|c| matrix[r][c] == 0))
        .collect();
    let zero_column: Vec<bool> = (0..columns)
        .map(|c| (0..rows).any(|r| matrix[r][c] == 0))
        .collect();

    (0..rows)
        .map(|r| {
            (0..columns)
                .map(|c| {
                    if zero_row[r] || zero_column[c] {
                        0
                    } else {
                        matrix[r][c]
                    }
                })
                .collect()
        })
        .collect()
}

fn zeroed(matrix: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let mut result = matrix.to_vec();
    Solution::set_zeroes(&mut result);
    result
}

fn check(matrix: &[Vec<i32>]) {
    let got = zeroed(matrix);
    let want = reference(matrix);

    assert_eq!(got.len(), matrix.len(), "row count changed for {matrix:?}");
    for row in &got {
        assert_eq!(row.len(), matrix[0].len(), "row width changed");
    }
    assert_eq!(got, want, "for {matrix:?}");
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

    fn below(&mut self, bound: usize) -> usize {
        (self.next_value() % bound as u64) as usize
    }

    /// One cell in `zero_in` is a zero; the rest are arbitrary non-zero values,
    /// drawn from the full i32 range so no value can serve as a sentinel.
    fn matrix(&mut self, rows: usize, columns: usize, zero_in: usize) -> Vec<Vec<i32>> {
        (0..rows)
            .map(|_| {
                (0..columns)
                    .map(|_| {
                        if self.below(zero_in) == 0 {
                            0
                        } else {
                            let v = self.next_value() as i32;
                            if v == 0 {
                                1
                            } else {
                                v
                            }
                        }
                    })
                    .collect()
            })
            .collect()
    }
}

fn grid(rows: &[&[i32]]) -> Vec<Vec<i32>> {
    rows.iter().map(|r| r.to_vec()).collect()
}

#[test]
fn example_1() {
    assert_eq!(
        zeroed(&grid(&[&[1, 1, 1], &[1, 0, 1], &[1, 1, 1]])),
        grid(&[&[1, 0, 1], &[0, 0, 0], &[1, 0, 1]])
    );
}

#[test]
fn example_2() {
    assert_eq!(
        zeroed(&grid(&[&[0, 1, 2, 0], &[3, 4, 5, 2], &[1, 3, 1, 5]])),
        grid(&[&[0, 0, 0, 0], &[0, 4, 5, 0], &[0, 3, 1, 0]])
    );
}

/// No zero anywhere means nothing changes.
#[test]
fn a_matrix_without_zeroes_is_untouched() {
    let m = grid(&[&[1, 2, 3], &[4, 5, 6]]);
    assert_eq!(zeroed(&m), m);
}

/// One zero clears everything that shares its row or column.
#[test]
fn a_single_zero() {
    assert_eq!(zeroed(&grid(&[&[0]])), grid(&[&[0]]));
    assert_eq!(zeroed(&grid(&[&[7]])), grid(&[&[7]]));
    assert_eq!(
        zeroed(&grid(&[&[1, 2], &[3, 0]])),
        grid(&[&[1, 0], &[0, 0]])
    );
}

/// A zero anywhere in a single row or single column clears the whole thing.
#[test]
fn single_row_and_single_column() {
    assert_eq!(zeroed(&grid(&[&[1, 2, 0, 4]])), grid(&[&[0, 0, 0, 0]]));
    assert_eq!(zeroed(&grid(&[&[1, 2, 3, 4]])), grid(&[&[1, 2, 3, 4]]));
    assert_eq!(
        zeroed(&grid(&[&[1], &[0], &[3]])),
        grid(&[&[0], &[0], &[0]])
    );
}

/// Every corner in turn. A solution that reuses the first row or column as its
/// own bookkeeping has to treat these specially, and this is where it breaks.
#[test]
fn a_zero_in_each_corner() {
    let base = grid(&[&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]]);

    for (r, c) in [(0usize, 0usize), (0, 2), (2, 0), (2, 2)] {
        let mut m = base.clone();
        m[r][c] = 0;
        check(&m);
    }
}

/// Zeros already spanning the first row and first column at once.
#[test]
fn zeroes_along_the_first_row_and_column() {
    check(&grid(&[&[0, 1, 1], &[1, 1, 1], &[1, 1, 1]]));
    check(&grid(&[&[0, 0, 0], &[1, 1, 1], &[1, 1, 1]]));
    check(&grid(&[&[0, 1, 1], &[0, 1, 1], &[0, 1, 1]]));
    check(&grid(&[&[1, 1, 0], &[1, 1, 1], &[0, 1, 1]]));
}

/// Everything is already zero, and everything becomes zero.
#[test]
fn all_zeroes() {
    for (rows, columns) in [(1usize, 1usize), (1, 5), (5, 1), (3, 4), (7, 7)] {
        let m = vec![vec![0; columns]; rows];
        assert_eq!(zeroed(&m), m, "{rows} x {columns}");
    }
}

/// The extremes of the value range appear as ordinary cells, so no value can be
/// borrowed as a marker for "this was cleared".
#[test]
fn extreme_values_are_ordinary_cells() {
    let m = grid(&[
        &[i32::MIN, i32::MAX, 1],
        &[i32::MAX, 0, i32::MIN],
        &[1, i32::MIN, i32::MAX],
    ]);
    assert_eq!(
        zeroed(&m),
        grid(&[&[i32::MIN, 0, 1], &[0, 0, 0], &[1, 0, i32::MAX]])
    );

    let m = grid(&[&[i32::MIN, i32::MAX], &[i32::MAX, i32::MIN]]);
    assert_eq!(zeroed(&m), m);
}

/// Generated matrices across a range of shapes and zero densities.
#[test]
fn matches_reference_on_generated_matrices() {
    let mut rng = Rng(0x243F_6A88_85A3_08D3);

    for rows in 1..=6 {
        for columns in 1..=6 {
            for zero_in in [2usize, 4, 12, 40] {
                for _ in 0..15 {
                    let m = rng.matrix(rows, columns, zero_in);
                    check(&m);
                }
            }
        }
    }
}

/// Zeros are only ever added, never removed. Note the transformation is *not*
/// idempotent — clearing [[0, a], [b, c]] gives [[0, 0], [0, c]], and a second
/// pass would see those new zeros and clear the rest — which is exactly why the
/// decision must be made from the untouched input.
#[test]
fn zeroes_are_never_removed() {
    let mut rng = Rng(0x1357_9BDF_2468_ACE0);

    for rows in 1..=5 {
        for columns in 1..=5 {
            for _ in 0..20 {
                let m = rng.matrix(rows, columns, 6);
                let got = zeroed(&m);

                for r in 0..rows {
                    for c in 0..columns {
                        if m[r][c] == 0 {
                            assert_eq!(got[r][c], 0, "({r},{c}) lost its zero in {m:?}");
                        }
                    }
                }
            }
        }
    }
}

/// Restates the rule cell by cell, against the untouched input: a cell is zero
/// exactly when its original row or column held one.
#[test]
fn every_cell_follows_the_rule() {
    let mut rng = Rng(0xDEAD_BEEF_CAFE_F00D);

    for _ in 0..200 {
        let (rows, columns) = (1 + rng.below(7), 1 + rng.below(7));
        let m = rng.matrix(rows, columns, 8);
        let got = zeroed(&m);

        for r in 0..rows {
            for c in 0..columns {
                let cleared = (0..columns).any(|k| m[r][k] == 0) || (0..rows).any(|k| m[k][c] == 0);

                if cleared {
                    assert_eq!(got[r][c], 0, "({r},{c}) should be cleared in {m:?}");
                } else {
                    assert_eq!(got[r][c], m[r][c], "({r},{c}) should survive in {m:?}");
                }
            }
        }
    }
}

/// 200 x 200 is the constraint's maximum on both axes.
#[test]
fn largest_allowed_matrix() {
    let mut rng = Rng(0x5A5A_1111_2222_3333);

    check(&rng.matrix(200, 200, 500));
    check(&rng.matrix(200, 1, 50));
    check(&rng.matrix(1, 200, 50));

    let mut m = vec![vec![9; 200]; 200];
    m[199][199] = 0;
    check(&m);

    let m = vec![vec![9; 200]; 200];
    assert_eq!(zeroed(&m), m);
}
