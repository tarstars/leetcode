use crate::Solution;

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let n = nums.len() as i32;

        let mut p = (n - 2) as i32;
        while p >= 0 && nums[p as usize] >= nums[(p + 1) as usize] {
            p -= 1;
        }

        let mut left_swap: i32 = 0;
        let mut right_swap: i32 = n - 1;

        if p >= 0 {
            let mut biggest_ind = n - 1;
            while nums[biggest_ind as usize] <= nums[p as usize] {
                biggest_ind -= 1;
            }
            (nums[p as usize], nums[biggest_ind as usize]) =
                (nums[biggest_ind as usize], nums[p as usize]);
            left_swap = p + 1;
        }

        while left_swap < right_swap {
            (nums[left_swap as usize], nums[right_swap as usize]) =
                (nums[right_swap as usize], nums[left_swap as usize]);
            left_swap += 1;
            right_swap -= 1;
        }
    }
}
