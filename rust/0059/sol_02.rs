use crate::Solution;

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let size = usize::try_from(n).expect("n must be non-negative");
        let mut matrix = vec![vec![0; size]; size];
        let directions = [(0_isize, 1_isize), (1, 0), (0, -1), (-1, 0)];
        let (mut row, mut column) = (0_isize, 0_isize);
        let mut direction = 0;

        for value in 1..=n * n {
            matrix[row as usize][column as usize] = value;
            if value == n * n {
                break;
            }

            let (row_step, column_step) = directions[direction];
            let (next_row, next_column) = (row + row_step, column + column_step);
            let next_is_open = next_row >= 0
                && next_column >= 0
                && (next_row as usize) < size
                && (next_column as usize) < size
                && matrix[next_row as usize][next_column as usize] == 0;

            if !next_is_open {
                direction = (direction + 1) % directions.len();
            }

            row += directions[direction].0;
            column += directions[direction].1;
        }

        matrix
    }
}
