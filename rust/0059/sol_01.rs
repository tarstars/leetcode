use crate::Solution;

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let size = usize::try_from(n).expect("n must be non-negative");
        let mut matrix = vec![vec![0; size]; size];
        let (mut top, mut bottom) = (0, size);
        let (mut left, mut right) = (0, size);
        let mut next = 1;

        while top < bottom && left < right {
            for cell in &mut matrix[top][left..right] {
                *cell = next;
                next += 1;
            }
            top += 1;
            if top == bottom {
                break;
            }

            right -= 1;
            for row in &mut matrix[top..bottom] {
                row[right] = next;
                next += 1;
            }
            if left == right {
                break;
            }

            bottom -= 1;
            for cell in matrix[bottom][left..right].iter_mut().rev() {
                *cell = next;
                next += 1;
            }

            for row in matrix[top..bottom].iter_mut().rev() {
                row[left] = next;
                next += 1;
            }
            left += 1;
        }

        matrix
    }
}
