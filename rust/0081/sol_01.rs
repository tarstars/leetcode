use crate::Solution;

impl Solution {
    /// The canonical form: compare only `nums[lo]` with `nums[mid]`, and treat a
    /// tie between them as undecidable.
    ///
    /// This is the version most write-ups give, and it is worth knowing mainly
    /// as a contrast with sol_00. It asks one question where sol_00 asks two, so
    /// it reaches the undecidable branch more often — a tie on the left says
    /// nothing about the right, and `[1,1,1,2,3]` is a case this one gives up on
    /// while sol_00 still halves. On an all-equal array of 5000 elements it
    /// takes 5000 iterations against sol_00's 2500, since it can only shed
    /// `lo` where sol_00 sheds both ends.
    ///
    /// What it buys is brevity: three branches instead of five, and one
    /// comparison to reason about. The asymptotics are identical — O(log n) when
    /// values are distinct, O(n) when they are not.
    ///
    /// Shedding `lo` is safe because this branch is only reached after
    /// `nums[mid] != target`, and `nums[lo] == nums[mid]`, so `nums[lo]` cannot
    /// be the target either.
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        let (mut lo, mut hi) = (0i64, nums.len() as i64 - 1);

        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            let middle = nums[mid as usize];

            if middle == target {
                return true;
            }

            let left = nums[lo as usize];

            if left == middle {
                // Undecidable: the pivot could be on either side.
                lo += 1;
            } else if left < middle {
                // The left half is sorted, so the target's presence there is
                // decided by a range check.
                if left <= target && target < middle {
                    hi = mid - 1;
                } else {
                    lo = mid + 1;
                }
            } else {
                // The pivot lies in the left half, so the right half is sorted.
                if middle < target && target <= nums[hi as usize] {
                    lo = mid + 1;
                } else {
                    hi = mid - 1;
                }
            }
        }

        false
    }
}
