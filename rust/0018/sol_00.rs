use std::collections::HashSet;

use crate::Solution;

impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if nums.len() < 4 {
            return vec![];
        }
        nums.sort();

        let mut quadro: HashSet<[i32; 4]> = HashSet::new();

        let n = nums.len();

        for p in 0..n-3 {
            for q in p + 1..n-2 {
                let mut s = n - 1;
                for r in q + 1..n-1 {
                    loop {
                        let t = nums[p] as i64 + nums[q] as i64 + nums[r] as i64 + nums[s] as i64;
                        if t > target as i64 && s - 1 > r {
                            s -= 1;
                            continue;
                        }
                        if t == target as i64 && s != r {
                            quadro.insert([nums[p], nums[q], nums[r], nums[s]]);
                        }
                        break
                    }
                }
            }
        }

        quadro.into_iter().map(|q| q.to_vec()).collect()
    }
}
