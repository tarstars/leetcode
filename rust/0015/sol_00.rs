use crate::Solution;

use std::collections::HashSet;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();


        let mut res: HashSet<[i32; 3]> = HashSet::new();

        for q in 1..nums.len() - 1 {
            let mut p = 0;
            let mut r = nums.len() - 1;

            loop {
                let t = nums[p] + nums[q] + nums[r];
                if t > 0 {
                    if r - 1 > q {
                        r -= 1;
                        continue
                    }
                    break
                }
                if t < 0 {
                    if p + 1 < q {
                        p += 1;
                        continue
                    }
                    break
                }
                res.insert([nums[p], nums[q], nums[r]]);
                if p + 1 < q {
                    p += 1;
                } else if r - 1 > q {
                    r -= 1;
                } else {
                    break
                }
            }
        }

        res.into_iter().map(|t| t.to_vec()).collect()
    }
}
