use crate::Solution;

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let Some(pivot) = nums.windows(2).rposition(|pair| pair[0] < pair[1]) else {
            nums.reverse();
            return;
        };

        let successor = (pivot + 1..nums.len())
            .rfind(|&index| nums[index] > nums[pivot])
            .expect("a pivot always has a larger value in its suffix");

        nums.swap(pivot, successor);
        nums[pivot + 1..].reverse();
    }
}
