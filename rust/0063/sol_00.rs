use crate::Solution;

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let w = obstacle_grid[0].len();
        let h = obstacle_grid.len();
        let mut column = vec![0; h];
        let mut next = vec![0; h];
        column[0] = 1 - obstacle_grid[0][0];
        for p in 1..h {
            column[p] = column[p - 1] * (1 - obstacle_grid[p - 1][0]);
        }

        for q in 1..w {
            next[0] = column[0] * (1 - obstacle_grid[0][q - 1]);
            for p in 1..h {
                next[p] = column[p] * (1 - obstacle_grid[p][q - 1])
                    + next[p - 1] * (1 - obstacle_grid[p - 1][q]);
            }
            (column, next) = (next, column);
        }

        (*column.last().unwrap()) * (1 - obstacle_grid[h - 1][w - 1])
    }
}
