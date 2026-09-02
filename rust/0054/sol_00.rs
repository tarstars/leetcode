use crate::Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut a: Vec<i32> = Vec::new();
        let h = matrix.len();
        let w = matrix[0].len();

        let mut l = 0;
        let mut r = w - 1;
        let mut t = 0;
        let mut b = h - 1;

        while r >= l && b >= t {
            for q in l..=r {
                a.push(matrix[t][q]);
            }
            t += 1;
            if b < t {
                break;
            }

            for p in t..=b {
                a.push(matrix[p][r]);
            }
            if r == 0 {
                break;
            }
            r -= 1;
            if r < l {
                break;
            }

            for q in (l..=r).rev() {
                a.push(matrix[b][q]);
            }
            b -= 1;
            if b < t {
                break;
            }

            for p in (t..=b).rev() {
                a.push(matrix[p][l]);
            }
            l += 1;
        }

        a
    }
}
