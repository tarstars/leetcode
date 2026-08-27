use crate::Solution;

fn binary_search(mut left: usize, mut right: usize, p: impl Fn(usize) -> bool) -> usize {
    while right > left {
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
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let lower_bound = binary_search(0, nums.len(), |x| nums[x] >= target);
        let upper_bound = binary_search(0, nums.len(), |x| nums[x] > target);

        if lower_bound == upper_bound {
            vec![-1, -1]
        } else {
            vec![lower_bound as i32, (upper_bound - 1) as i32]
        }
    }
}
