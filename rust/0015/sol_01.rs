use crate::Solution;

impl Solution {
    /// Outer loop fixes the *smallest* element, then one two-pointer sweep
    /// covers the rest of the array. Duplicates are skipped by comparing
    /// neighbours in the sorted input, so no hash set is needed.
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();

        let n = nums.len();
        let mut res: Vec<Vec<i32>> = Vec::new();

        for p in 0..n.saturating_sub(2) {
            // Sorted input: once the smallest element is positive, so is every sum.
            if nums[p] > 0 {
                break;
            }
            // Same value as the previous p would repeat that p's whole sweep.
            if p > 0 && nums[p] == nums[p - 1] {
                continue;
            }

            let (mut q, mut r) = (p + 1, n - 1);
            while q < r {
                let t = nums[p] + nums[q] + nums[r];
                if t < 0 {
                    q += 1;
                } else if t > 0 {
                    r -= 1;
                } else {
                    res.push(vec![nums[p], nums[q], nums[r]]);
                    q += 1;
                    r -= 1;
                    while q < r && nums[q] == nums[q - 1] {
                        q += 1;
                    }
                    while q < r && nums[r] == nums[r + 1] {
                        r -= 1;
                    }
                }
            }
        }

        res
    }
}
