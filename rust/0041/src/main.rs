#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    for nums in [vec![1, 2, 0], vec![3, 4, -1, 1], vec![7, 8, 9, 11, 12]] {
        let answer = Solution::first_missing_positive(nums.clone());
        println!("{nums:?} -> {answer}");
    }
}

#[cfg(test)]
mod tests;
