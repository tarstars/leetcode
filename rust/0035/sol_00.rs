use crate::Solution;

fn binary_search(mut left: usize, mut right: usize, p: impl Fn(usize) -> bool) -> usize {
    while left < right {
        let mid = left + (right - left) / 2;
        if p(mid) {
            right = mid
        } else {
            left = mid + 1
        }
    }

    right
}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        binary_search(0, nums.len(), |x| nums[x] >= target) as i32
    }
}
