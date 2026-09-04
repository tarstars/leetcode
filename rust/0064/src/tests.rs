use super::*;

/// Enumerates every path and keeps the cheapest. Exponential, so it is only
/// used on tiny grids, but it follows the problem statement literally and
/// shares no logic with a dynamic-programming solution.
fn brute_force(grid: &[Vec<i32>]) -> i64 {
    fn walk(grid: &[Vec<i32>], row: usize, column: usize) -> i64 {
        let (m, n) = (grid.len(), grid[0].len());
        let here = grid[row][column] as i64;

        if row == m - 1 && column == n - 1 {
            return here;
        }

        let mut best = i64::MAX;
        if row + 1 < m {
            best = best.min(walk(grid, row + 1, column));
        }
        if column + 1 < n {
            best = best.min(walk(grid, row, column + 1));
        }

        here + best
    }

    walk(grid, 0, 0)
}

/// The same minimum by dynamic programming, in i64. Used where the grid is too
/// large to enumerate paths.
fn reference(grid: &[Vec<i32>]) -> i64 {
    let n = grid[0].len();
    let mut row = vec![i64::MAX; n];
    row[0] = 0;

    for grid_row in grid {
        row[0] += grid_row[0] as i64;
        for column in 1..n {
            row[column] = row[column].min(row[column - 1]) + grid_row[column] as i64;
        }
    }

    row[n - 1]
}

fn min_sum(grid: &[Vec<i32>]) -> i64 {
    Solution::min_path_sum(grid.to_vec()) as i64
}

/// A deterministic generator, so a failure always reproduces.
struct Rng(u64);

impl Rng {
    fn next_value(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        self.0 >> 33
    }

    fn below(&mut self, bound: u64) -> u64 {
        self.next_value() % bound
    }

    fn grid(&mut self, m: usize, n: usize, max_cell: u64) -> Vec<Vec<i32>> {
        (0..m)
            .map(|_| (0..n).map(|_| self.below(max_cell + 1) as i32).collect())
            .collect()
    }
}

#[test]
fn example_1() {
    assert_eq!(min_sum(&[vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]), 7);
}

#[test]
fn example_2() {
    assert_eq!(min_sum(&[vec![1, 2, 3], vec![4, 5, 6]]), 12);
}

/// A single cell is both start and destination, so its own value is the answer.
#[test]
fn single_cell() {
    assert_eq!(min_sum(&[vec![7]]), 7);
    assert_eq!(min_sum(&[vec![0]]), 0);
}

/// With one row or one column there is no choice: every cell is on the path.
#[test]
fn single_row_or_column() {
    assert_eq!(min_sum(&[vec![1, 2, 3, 4]]), 10);
    assert_eq!(min_sum(&[vec![1], vec![2], vec![3], vec![4]]), 10);
}

/// Zero is a legal cell value, and a grid of them costs nothing.
#[test]
fn all_zeros() {
    for (m, n) in [(1, 1), (1, 9), (9, 1), (4, 6), (7, 7)] {
        assert_eq!(min_sum(&vec![vec![0; n]; m]), 0, "{m} x {n}");
    }
}

/// Every path visits exactly m + n - 1 cells, so a uniform grid has only one
/// possible total no matter which route is taken.
#[test]
fn uniform_grid_depends_only_on_the_path_length() {
    for (m, n) in [(1, 1), (3, 5), (5, 3), (8, 8)] {
        for value in [0, 1, 7, 200] {
            let expected = value as i64 * (m + n - 1) as i64;
            assert_eq!(min_sum(&vec![vec![value; n]; m]), expected, "{m} x {n}");
        }
    }
}

/// Taking the cheaper of the two next cells at each step walks into the
/// expensive region: greedy scores 105 here, the optimum is 9.
#[test]
fn greedy_choice_is_not_optimal() {
    let grid = vec![vec![1, 2, 2], vec![1, 99, 2], vec![99, 99, 2]];
    assert_eq!(min_sum(&grid), 9);
}

/// The two references must agree with each other before either is trusted.
#[test]
fn the_references_agree_on_small_grids() {
    let mut rng = Rng(0x5EED);

    for m in 1..=5usize {
        for n in 1..=5usize {
            for _ in 0..20 {
                let grid = rng.grid(m, n, 20);
                assert_eq!(brute_force(&grid), reference(&grid), "{grid:?}");
            }
        }
    }
}

/// Random small grids against path enumeration.
#[test]
fn matches_brute_force_on_small_grids() {
    let mut rng = Rng(0xC0FFEE);

    for m in 1..=5usize {
        for n in 1..=5usize {
            for _ in 0..40 {
                let grid = rng.grid(m, n, 200);
                assert_eq!(min_sum(&grid), brute_force(&grid), "{grid:?}");
            }
        }
    }
}

/// Larger random grids, including very lopsided ones, against the reference.
#[test]
fn matches_reference_on_larger_grids() {
    let mut rng = Rng(0xBADC0DE);

    for (m, n) in [
        (1, 200),
        (200, 1),
        (2, 137),
        (137, 2),
        (17, 41),
        (60, 60),
        (200, 199),
    ] {
        let grid = rng.grid(m, n, 200);
        assert_eq!(min_sum(&grid), reference(&grid), "{m} x {n}");
    }
}

/// Many cells share the same value, which is where a comparison written the
/// wrong way round (< versus <=) tends to show up.
#[test]
fn grids_with_many_ties() {
    let mut rng = Rng(0x71E5);

    for (m, n) in [(4, 4), (9, 6), (50, 50)] {
        let grid = rng.grid(m, n, 1);
        assert_eq!(
            min_sum(&grid),
            reference(&grid),
            "{m} x {n} of zeros and ones"
        );
    }
}

/// 200 x 200 at the value ceiling — the constraint's maximum on both axes.
/// Every path costs 200 * (200 + 200 - 1); enumerating paths to discover that
/// would take longer than the age of the universe.
#[test]
fn largest_allowed_grid() {
    assert_eq!(min_sum(&vec![vec![200; 200]; 200]), 79_800);

    let mut rng = Rng(0xFEEDFACE);
    let grid = rng.grid(200, 200, 200);
    assert_eq!(min_sum(&grid), reference(&grid));
}
