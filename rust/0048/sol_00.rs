use crate::Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let height = matrix.len();
        let width = matrix[0].len();

        for p0 in 0..(height / 2) {
            for q0 in 0..((width + 1) / 2) {
                let (p1, q1) = (q0, height - 1 - p0);
                let (p2, q2) = (height - 1 - p0, width - 1 - q0);
                let (p3, q3) = (width - 1 - q0, p0);

                (matrix[p1][q1], matrix[p2][q2], matrix[p3][q3], matrix[p0][q0]) =
                (matrix[p0][q0], matrix[p1][q1], matrix[p2][q2], matrix[p3][q3]);
            }
        }
    }
}
