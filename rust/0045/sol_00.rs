use crate::Solution;

use std::cmp::min;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp: Vec<i32> = vec![(n + 1) as i32; n];
        dp[0] = 0;

        for q in 0..n {
            for p in q..=min(n - 1, (q + nums[q] as usize)) {
                dp[p] = min(dp[p], dp[q] + 1);
            }
        }

        if *dp.last().unwrap() == (n + 1) as i32 {-1} else {*dp.last().unwrap()}
    }
}
