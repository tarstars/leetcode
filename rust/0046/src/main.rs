#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    for nums in [vec![1, 2, 3], vec![0, 1]] {
        let permutations = Solution::permute(nums.clone());
        println!("{nums:?} -> {permutations:?}");
    }
}

#[cfg(test)]
mod tests;
