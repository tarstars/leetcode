#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    for nums in [vec![1, 2, 2], vec![0], vec![4, 4, 4, 1, 4]] {
        println!("{nums:?} -> {:?}", Solution::subsets_with_dup(nums.clone()));
    }
}

#[cfg(test)]
mod tests;
