use crate::Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut e_0 = 0usize;
        let mut e_1 = 0usize;
        let mut e_2 = 0usize;

        for p in 0..nums.len() {
            match nums[p] {
                2 => {
                    nums[e_2] = 2;
                    e_2 += 1;
                }
                1 => {
                    nums[e_2] = 2;
                    e_2 += 1;
                    nums[e_1] = 1;
                    e_1 += 1;
                }
                0 => {
                    nums[e_2] = 2;
                    e_2 += 1;
                    nums[e_1] = 1;
                    e_1 += 1;
                    nums[e_0] = 0;
                    e_0 += 1;
                }
                _ => {}
            }
        }
    }
}
