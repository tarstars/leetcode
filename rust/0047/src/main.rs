#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    for nums in [vec![1, 1, 2], vec![1, 2, 3]] {
        let permutations = Solution::permute_unique(nums.clone());
        println!("{nums:?} -> {permutations:?}");
    }
}

#[cfg(test)]
mod tests;
