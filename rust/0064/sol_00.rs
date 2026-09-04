use crate::Solution;

use std::cmp::min;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut row_curr = grid[0].clone();
        let h = grid.len();
        let w = grid[0].len();

        for q in 1..w {
            row_curr[q] += row_curr[q - 1];
        }

        let mut row_next = vec![0; w];

        for p in 1..h {
            row_next[0] = row_curr[0] + grid[p][0];
            for q in 1..w {
                row_next[q] = min(row_curr[q], row_next[q - 1]) + grid[p][q];
            }
            (row_curr, row_next) = (row_next, row_curr);
        }

        *row_curr.last().unwrap()
    }
}
