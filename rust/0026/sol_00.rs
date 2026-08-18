use crate::Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let n = nums.len();
        let mut p = 0;
        let mut q = 0;

        while q < n {
            nums[p] = nums[q];
            while q < n && nums[q] == nums[p] {
                q += 1;
            }
            p += 1;
        }

        p as i32
    }
}
