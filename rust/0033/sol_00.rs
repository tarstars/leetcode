use crate::Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while right - left > 1 {
            let mid = left + (right - left) / 2;

            if (nums[left] <= target && target <= nums[mid])
                || (nums[left] > nums[mid]) && (nums[mid] >= target || target >= nums[left])
            {
                right = mid
            } else {
                left = mid + 1
            }
        }

        if nums[left] == target {
            return left as i32;
        }

        if nums[right] == target {
            return right as i32;
        }

        -1
    }
}
