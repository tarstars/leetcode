use crate::Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut power_set: Vec<Vec<i32>> = vec![vec![]];

        for v in nums {
            let mut contains_v: Vec<Vec<i32>> = Vec::new();
            for subset in &power_set {
                let mut new_subset = subset.clone();
                new_subset.push(v);
                contains_v.push(new_subset);
            }
            power_set.extend(contains_v);
        }

        power_set
    }
}
