use crate::Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        let n = nums.len();
        let mut l = 0usize;
        let mut r = n - 1;

        while r - l > 1 {
            let m = l + (r - l) / 2;
            if nums[l] < nums[m] {
                if nums[l] <= target && target <= nums[m] {
                    r = m;
                } else {
                    l = m;
                }
            } else if nums[l] > nums[m] {
                if target >= nums[l] || target <= nums[m] {
                    r = m;
                } else {
                    l = m;
                }
            } else if nums[m] < nums[r] {
                if nums[m] <= target && target <= nums[r] {
                    l = m;
                } else {
                    r = m;
                }
            } else if nums[m] > nums[r] {
                if nums[m] <= target || target <= nums[r] {
                    l = m;
                } else {
                    r = m;
                }
            } else {
                l = l + 1;
                r = r - 1;
            }
        }

        nums[l] == target || nums[r] == target
    }
}
