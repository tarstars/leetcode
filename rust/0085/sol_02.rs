use crate::Solution;

impl Solution {
    /// A third approach, alongside the run-length scan in `sol_00` and the
    /// per-row histogram in `sol_01`.
    ///
    /// Still unimplemented anywhere in this problem:
    ///
    /// * **Left / right / height DP.** Sweep rows top to bottom keeping three
    ///   arrays, one entry per column: `height` (the run of `'1'`s ending at
    ///   this row, same as in `sol_01`), `left` (the leftmost column that run
    ///   can extend to) and `right` (one past the rightmost). Each row updates
    ///   `height` and `left` in a left-to-right pass and `right` in a
    ///   right-to-left pass, and the answer for a column is
    ///   `height[q] * (right[q] - left[q])`. O(rows * cols) with no stack —
    ///   the subtle part is that `left` and `right` must be *reset* at a `'0'`
    ///   and otherwise carried across rows, narrowing but never widening.
    /// * **Prefix-sum brute force.** Build a 2D prefix sum of the ones, then a
    ///   rectangle is all ones exactly when its sum equals its area, so every
    ///   rectangle is testable in O(1). O(rows^2 * cols^2) — far too slow at
    ///   200 x 200, so the large tests will time out, but it is the most direct
    ///   statement of the problem there is.
    ///
    /// Run it against the shared suite with `cargo test --bin sol_02`.
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let _ = matrix;
        todo!("implement the third approach to Solution::maximal_rectangle")
    }
}
