use crate::Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn backtrack(
            nums: &[i32],
            start: usize,
            subset: &mut Vec<i32>,
            subsets: &mut Vec<Vec<i32>>,
        ) {
            subsets.push(subset.clone());

            for index in start..nums.len() {
                subset.push(nums[index]);
                backtrack(nums, index + 1, subset, subsets);
                subset.pop();
            }
        }

        let mut subsets = Vec::with_capacity(1 << nums.len());
        backtrack(&nums, 0, &mut Vec::new(), &mut subsets);
        subsets
    }
}
