#![allow(clippy::ptr_arg)] // LeetCode requires `&mut Vec<Vec<i32>>`.

#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    Solution::rotate(&mut matrix);
    println!("{matrix:?}");
}

#[cfg(test)]
mod tests;
