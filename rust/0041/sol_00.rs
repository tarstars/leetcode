use crate::Solution;

impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;

        for start_ind in 0..n {
            let mut v = nums[start_ind as usize];
            if v > 0 {
                while v > 0 && v <= n && nums[(v - 1) as usize] != v {
                    let nv = nums[(v - 1) as usize];
                    nums[(v - 1) as usize] = v;
                    v = nv;
                }
            }
        }

        let mut cand = 1;

        while cand <= n && cand == nums[(cand - 1) as usize] {
            cand += 1;
        }

        cand
    }
}
