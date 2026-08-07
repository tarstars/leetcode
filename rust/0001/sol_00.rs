use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let n = nums.len();
        let mut t2i: HashMap<i32, usize> = HashMap::new();
        for p in 0..n {
            t2i.insert(target - nums[p], p);
        }
        for p in 0..n {
            if let Some(q) = t2i.get(&nums[p])  {
                if *q != p {
                    return vec![p as i32, *q as i32];
                }
            } 
        }
        vec![]
    }
}
