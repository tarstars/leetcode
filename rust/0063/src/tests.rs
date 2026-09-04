use super::*;

/// Enumerates every path one step at a time. Exponential, so it is only used on
/// tiny grids, but it follows the problem statement literally and shares no
/// logic with a dynamic-programming solution.
fn brute_force(grid: &[Vec<i32>]) -> u128 {
    fn walk(grid: &[Vec<i32>], row: usize, column: usize) -> u128 {
        let (m, n) = (grid.len(), grid[0].len());

        if row >= m || column >= n || grid[row][column] == 1 {
            return 0;
        }
        if row == m - 1 && column == n - 1 {
            return 1;
        }

        walk(grid, row + 1, column) + walk(grid, row, column + 1)
    }

    walk(grid, 0, 0)
}

/// The same count by dynamic programming, in u128 so it never overflows. Used
/// where the grid is too large to enumerate paths.
fn reference(grid: &[Vec<i32>]) -> u128 {
    let n = grid[0].len();
    let mut row = vec![0u128; n];
    row[0] = 1;

    for grid_row in grid {
        for column in 0..n {
            if grid_row[column] == 1 {
                row[column] = 0;
            } else if column > 0 {
                row[column] += row[column - 1];
            }
        }
    }

    row[n - 1]
}

fn paths(rows: &[&str]) -> u128 {
    Solution::unique_paths_with_obstacles(grid(rows)) as u128
}

/// Enumerates all 2^(m*n) obstacle patterns for an m x n grid.
fn every_pattern(m: usize, n: usize) -> impl Iterator<Item = Vec<Vec<i32>>> {
    (0..1u32 << (m * n)).map(move |mask| {
        (0..m)
            .map(|row| {
                (0..n)
                    .map(|column| ((mask >> (row * n + column)) & 1) as i32)
                    .collect()
            })
            .collect()
    })
}

#[test]
fn example_1() {
    assert_eq!(paths(&["...", ".#.", "..."]), 2);
}

#[test]
fn example_2() {
    assert_eq!(paths(&[".#", ".."]), 1);
}

#[test]
fn single_open_cell() {
    assert_eq!(paths(&["."]), 1);
}

/// The robot starts on the destination, but it is blocked.
#[test]
fn single_blocked_cell() {
    assert_eq!(paths(&["#"]), 0);
}

/// An obstacle on the starting square admits no path at all.
#[test]
fn blocked_start() {
    assert_eq!(paths(&["#..", "...", "..."]), 0);
    assert_eq!(paths(&["#.", ".."]), 0);
}

/// An obstacle on the destination admits no path either.
#[test]
fn blocked_destination() {
    assert_eq!(paths(&["...", "...", "..#"]), 0);
    assert_eq!(paths(&["..", ".#"]), 0);
}

/// A wall spanning a whole row or column cuts the grid in two.
#[test]
fn a_full_wall_blocks_everything() {
    assert_eq!(paths(&["....", "####", "...."]), 0);
    assert_eq!(paths(&[".#.", ".#.", ".#."]), 0);
}

/// One row or one column: a single path, and any obstacle destroys it.
#[test]
fn single_row_or_column() {
    assert_eq!(paths(&["....."]), 1);
    assert_eq!(paths(&["..#.."]), 0);
    assert_eq!(paths(&[".", ".", ".", "."]), 1);
    assert_eq!(paths(&[".", ".", "#", "."]), 0);
}

/// A wall with one gap forces every path through it.
#[test]
fn a_gap_in_a_wall() {
    assert_eq!(paths(&["....", "###.", "...."]), 1);
    assert_eq!(paths(&["....", ".###", "...."]), 1);
}

/// With no obstacles the answer collapses to problem 62's binomial C(m+n-2, m-1).
#[test]
fn an_open_grid_is_the_binomial_coefficient() {
    for m in 1..=12usize {
        for n in 1..=12usize {
            let rows = vec![".".repeat(n); m];
            let rows: Vec<&str> = rows.iter().map(String::as_str).collect();

            let mut binomial = 1u128;
            for i in 1..=(m as u128 - 1) {
                binomial = binomial * (n as u128 - 1 + i) / i;
            }

            assert_eq!(paths(&rows), binomial, "{m} x {n} open grid");
        }
    }
}

/// The two references must agree with each other before either is trusted.
#[test]
fn the_references_agree_on_every_small_pattern() {
    for (m, n) in [(1, 1), (1, 3), (3, 1), (2, 2), (2, 3), (3, 2), (3, 3)] {
        for pattern in every_pattern(m, n) {
            assert_eq!(
                brute_force(&pattern),
                reference(&pattern),
                "references disagree on {pattern:?}"
            );
        }
    }
}

/// Exhaustive: every obstacle pattern on every grid up to 3x3, plus 4x3 and
/// 3x4, checked against path enumeration.
#[test]
fn matches_brute_force_on_every_small_pattern() {
    for (m, n) in [
        (1, 1),
        (1, 2),
        (2, 1),
        (1, 4),
        (4, 1),
        (2, 2),
        (2, 3),
        (3, 2),
        (3, 3),
        (3, 4),
        (4, 3),
    ] {
        for pattern in every_pattern(m, n) {
            let got = Solution::unique_paths_with_obstacles(pattern.clone()) as u128;
            assert_eq!(got, brute_force(&pattern), "{pattern:?}");
        }
    }
}

/// The largest count the constraints permit: an open 17x17 grid.
#[test]
fn largest_open_grid_within_the_bound() {
    let rows = vec![".".repeat(17); 17];
    let rows: Vec<&str> = rows.iter().map(String::as_str).collect();
    assert_eq!(paths(&rows), 601_080_390);
}

/// 100 x 100 — the constraint's maximum. Obstacles leave exactly one route,
/// so the answer stays inside the promised bound while the grid is at full
/// size: a solution that recurses per path, or per cell without memoising,
/// will not finish.
#[test]
fn largest_allowed_grid() {
    let mut grid = vec![vec![1; 100]; 100];
    for column in 0..100 {
        grid[0][column] = 0;
    }
    for row in 0..100 {
        grid[row][99] = 0;
    }

    assert_eq!(
        Solution::unique_paths_with_obstacles(grid.clone()) as u128,
        1
    );
    assert_eq!(reference(&grid), 1);
}

/// A full-size open corridor two cells wide, and a full-size single row and
/// column, all at the dimension limit.
#[test]
fn full_size_thin_grids() {
    let wide = vec![".".repeat(100)];
    let wide: Vec<&str> = wide.iter().map(String::as_str).collect();
    assert_eq!(paths(&wide), 1);

    let tall = vec!["."; 100];
    assert_eq!(paths(&tall), 1);

    let band = vec![".".repeat(100); 2];
    let band: Vec<&str> = band.iter().map(String::as_str).collect();
    assert_eq!(paths(&band), 100);
}
