use crate::Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = Vec::with_capacity(matrix.len() * matrix[0].len());
        let (mut top, mut bottom) = (0, matrix.len());
        let (mut left, mut right) = (0, matrix[0].len());

        while top < bottom && left < right {
            result.extend(matrix[top][left..right].iter().copied());
            top += 1;
            if top == bottom {
                break;
            }

            right -= 1;
            result.extend(matrix[top..bottom].iter().map(|row| row[right]));
            if left == right {
                break;
            }

            bottom -= 1;
            result.extend(matrix[bottom][left..right].iter().rev().copied());
            result.extend(matrix[top..bottom].iter().rev().map(|row| row[left]));
            left += 1;
        }

        result
    }
}
