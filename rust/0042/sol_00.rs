use crate::Solution;

use std::cmp::max;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let n = height.len();
        let mut l = 0;
        let mut r = n - 1;
        let mut l_max = height[l];
        let mut r_max = height[r];
        let mut l_vol = 0;
        let mut r_vol = 0;

        while l < r {
            if height[l] < height[r] {
                l += 1;
                l_max = max(l_max, height[l]);
                l_vol += l_max - height[l];
            } else {
                r -= 1;
                r_max = max(r_max, height[r]);
                r_vol += r_max - height[r];
            }
        }

        l_vol + r_vol
    }
}
