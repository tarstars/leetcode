#![allow(clippy::ptr_arg)] // LeetCode requires `&mut Vec<i32>`.

#[path = "../../sol_01.rs"]
mod sol_01;

struct Solution;

fn main() {
    for mut nums in [vec![1, 2, 3], vec![3, 2, 1], vec![1, 1, 5]] {
        let before = nums.clone();
        Solution::next_permutation(&mut nums);
        println!("{before:?} -> {nums:?}");
    }
}

#[cfg(test)]
#[path = "../tests.rs"]
mod tests;
