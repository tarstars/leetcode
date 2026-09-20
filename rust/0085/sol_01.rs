use crate::Solution;

use std::cmp::max;

fn max_area(a: &Vec<i32>) -> i32 {
    let mut best_val = 0;
    let mut st: Vec<usize> = Vec::new();
    let n = a.len();

    for p in 0..n {
        while !st.is_empty() && a[*st.last().unwrap()] > a[p] {
            let prev = st.pop().unwrap();
            let left_bound = if st.is_empty() {
                -1
            } else {
                *st.last().unwrap() as i32
            };
            best_val = max(best_val, (p as i32 - left_bound - 1) * a[prev]);
        }
        st.push(p);
    }

    while !st.is_empty() {
        let p = st.pop().unwrap();
        if let Some(pp) = st.last() {
            best_val = max(best_val, (n - pp - 1) as i32 * a[p]);
        } else {
            best_val = max(best_val, n as i32 * a[p])
        }
    }

    best_val
}

impl Solution {
    /// A second approach, for comparison with the run-length scan in `sol_00`.
    ///
    /// Still unimplemented anywhere in this problem:
    ///
    /// * **Histogram + stack per row.** Sweep rows top to bottom keeping one
    ///   running array of bar heights (a `'0'` resets its column to zero) and
    ///   call the problem-84 routine on each row. O(rows * cols).
    /// * **Left / right / height DP.** Per row, keep three arrays — the run
    ///   height at each column, and the leftmost and rightmost column that run
    ///   can reach — each updated in one pass. Also O(rows * cols), no stack.
    /// * **Prefix-sum brute force.** A rectangle is all ones exactly when its
    ///   sum equals its area, so a 2D prefix sum lets you test every rectangle
    ///   in O(1). O(rows^2 * cols^2), too slow at 200 x 200 but very direct.
    ///
    /// Run it against the shared suite with `cargo test --bin sol_01`.
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let mut best_val = 0;
        let h = matrix.len();
        let w = matrix[0].len();
        let mut row: Vec<i32> = vec![0; w];

        for p in 0..h {
            for q in 0..w {
                row[q] = (row[q] + 1) * if matrix[p][q] == '1' { 1 } else { 0 };
            }
            best_val = max(best_val, max_area(&row));
        }

        best_val
    }
}
