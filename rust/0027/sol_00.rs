use crate::Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let n = nums.len();
        let mut p = 0;

        for q in 0..n {
            if nums[q] != val {
                nums[p] = nums[q];
                p += 1;
            }
        }

        p as i32
    }
}
