use crate::Solution;

impl Solution {
    /// Two library searches instead of one hand-rolled search over a flattened
    /// index, which removes the index arithmetic — and with it the row-versus-
    /// width mix-up and the out-of-range sentinels — entirely.
    ///
    /// `partition_point` returns how many leading elements satisfy a predicate,
    /// which is exactly a lower bound. Since each row starts above the previous
    /// row's last value, the rows whose first element is at most `target` form a
    /// prefix, so the count is one past the only row that could hold it. A count
    /// of zero means `target` sits below the whole matrix.
    ///
    /// O(log m + log n), which is O(log(m * n)).
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let candidates = matrix.partition_point(|row| row[0] <= target);

        candidates > 0 && matrix[candidates - 1].binary_search(&target).is_ok()
    }
}
