use crate::Solution;

use std::cmp::max;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut max_reach = 0;

        for (p, v) in nums.iter().enumerate() {
            if max_reach < p {
                return false
            }            
            max_reach = max(max_reach, p + *v as usize);
        }

        true
    }
}
