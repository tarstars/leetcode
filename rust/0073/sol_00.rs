use crate::Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let h = matrix.len();
        let w = matrix[0].len();

        let mut zero_first_column = matrix[0][0] == 0i32;

        for p in 0..h {
            zero_first_column |= matrix[p][0] == 0;
            for q in 1..w {
                if matrix[p][q] == 0 {
                    matrix[0][q] = 0;
                    matrix[p][0] = 0;
                }
            }
        }

        for q in 1..w {
            if matrix[0][q] == 0 {
                for p in 0..h {
                    matrix[p][q] = 0;
                }
            }
        }

        for p in 0..h {
            if matrix[p][0] == 0 {
                for q in 0..w {
                    matrix[p][q] = 0;
                }
            }
            if zero_first_column {
                matrix[p][0] = 0;
            }
        }
    }
}
