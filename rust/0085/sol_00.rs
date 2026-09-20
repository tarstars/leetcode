use crate::Solution;

use std::cmp::{max, min};

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let h = matrix.len();
        let w = matrix[0].len();
        let mut run_height: Vec<Vec<i32>> = vec![vec![0; w]; h];
        let mut run_width: Vec<Vec<i32>> = vec![vec![0; w]; h];

        for p in 0..h {
            run_width[p][0] = if matrix[p][0] == '1' { 1 } else { 0 };
            for q in 1..w {
                run_width[p][q] =
                    (run_width[p][q - 1] + 1) * if matrix[p][q] == '1' { 1 } else { 0 };
            }
        }

        for q in 0..w {
            run_height[0][q] = if matrix[0][q] == '1' { 1 } else { 0 };
            for p in 1..h {
                run_height[p][q] =
                    (run_height[p - 1][q] + 1) * if matrix[p][q] == '1' { 1 } else { 0 };
            }
        }

        let mut the_best = 0;

        for p in 0..h {
            for q in 0..w {
                let mut min_width = run_width[p][q];
                for t in 0..run_height[p][q] {
                    min_width = min(min_width, run_width[p - t as usize][q]);
                    the_best = max(the_best, (t + 1) * min_width);
                }
            }
        }

        the_best
    }
}
