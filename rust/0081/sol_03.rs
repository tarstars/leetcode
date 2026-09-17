use crate::Solution;

/// Searches `nums[lo..=hi]`, which is a rotation of a sorted run.
///
/// The three cases are: the left half is visibly sorted, the right half is
/// visibly sorted, or neither can be established — and in that last case the
/// answer is simply to search both.
fn search_range(nums: &[i32], lo: usize, hi: usize, target: i32) -> bool {
    if lo > hi || hi >= nums.len() {
        return false;
    }

    let mid = lo + (hi - lo) / 2;
    if nums[mid] == target {
        return true;
    }

    let before = || mid > lo && search_range(nums, lo, mid - 1, target);
    let after = || search_range(nums, mid + 1, hi, target);

    if nums[lo] < nums[mid] {
        // Left half sorted: a range check decides which side to descend into.
        if nums[lo] <= target && target < nums[mid] {
            before()
        } else {
            after()
        }
    } else if nums[mid] < nums[hi] {
        // Right half sorted.
        if nums[mid] < target && target <= nums[hi] {
            after()
        } else {
            before()
        }
    } else {
        // Neither half can be shown sorted, so neither can be discarded.
        before() || after()
    }
}

impl Solution {
    /// Divide and conquer, which makes the complexity argument fall out of the
    /// recurrence instead of being reasoned about separately.
    ///
    /// Where sol_00 and sol_01 handle the undecidable case by shrinking the
    /// window a step at a time, this one simply recurses into *both* halves —
    /// which is what "cannot decide" honestly means. The cost is then visible in
    /// the shape of the recursion rather than hidden in a loop:
    ///
    /// ```text
    ///   decidable everywhere:  T(n) = T(n/2) + O(1)   = O(log n)
    ///   undecidable everywhere: T(n) = 2 T(n/2) + O(1) = O(n)
    /// ```
    ///
    /// The second line is the follow-up question's answer, derived rather than
    /// asserted. Real inputs sit between the two, and the recursion degrades
    /// smoothly: each undecidable probe doubles the work at that level only, so
    /// a handful of duplicates costs a constant factor, not the bound.
    ///
    /// `||` short-circuits, so the second half is skipped whenever the first one
    /// finds the target.
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        if nums.is_empty() {
            return false;
        }

        search_range(&nums, 0, nums.len() - 1, target)
    }
}
