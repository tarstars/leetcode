use crate::Solution;

impl Solution {
    /// Treat the duplicates as a preprocessing problem, not an algorithmic one.
    ///
    /// Peel matching values off the tail until `nums[lo] != nums[hi]`. After
    /// that the window is a rotation whose two ends differ, which is enough for
    /// the plain `nums[lo] <= nums[mid]` test to identify the sorted half — so
    /// the loop below is problem 33's algorithm, unchanged and unaware that
    /// duplicates ever existed.
    ///
    /// The appeal is separation of concerns: one loop deals with the new
    /// wrinkle, and the tested, familiar routine is reused verbatim. The cost is
    /// the same as everywhere else — the peeling loop is O(n) on an all-equal
    /// array, which is exactly where the guarantee has to be given up.
    ///
    /// Note the peel drops from the *tail*. Dropping from the head would be
    /// wrong: `nums[lo]` is the one value known to sit at or after the pivot,
    /// and the range checks below are anchored on it.
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        let (mut lo, mut hi) = (0i64, nums.len() as i64 - 1);

        // Restore the precondition problem 33 relies on.
        while lo < hi && nums[lo as usize] == nums[hi as usize] {
            hi -= 1;
        }

        // From here on: problem 33, verbatim.
        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            let middle = nums[mid as usize];

            if middle == target {
                return true;
            }

            if nums[lo as usize] <= middle {
                if nums[lo as usize] <= target && target < middle {
                    hi = mid - 1;
                } else {
                    lo = mid + 1;
                }
            } else if middle < target && target <= nums[hi as usize] {
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }

        false
    }
}
