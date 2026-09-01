use crate::Solution;

fn search(
    nums: &[i32],
    used: &mut [bool],
    permutation: &mut Vec<i32>,
    results: &mut Vec<Vec<i32>>,
) {
    if permutation.len() == nums.len() {
        results.push(permutation.clone());
        return;
    }

    for index in 0..nums.len() {
        if used[index] || (index > 0 && nums[index] == nums[index - 1] && !used[index - 1]) {
            continue;
        }

        used[index] = true;
        permutation.push(nums[index]);
        search(nums, used, permutation, results);
        permutation.pop();
        used[index] = false;
    }
}

impl Solution {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();

        let mut results = Vec::new();
        search(
            &nums,
            &mut vec![false; nums.len()],
            &mut Vec::new(),
            &mut results,
        );
        results
    }
}
