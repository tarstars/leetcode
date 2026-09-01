#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    for nums in [
        vec![-2, 1, -3, 4, -1, 2, 1, -5, 4],
        vec![1],
        vec![5, 4, -1, 7, 8],
    ] {
        let sum = Solution::max_sub_array(nums.clone());
        println!("{nums:?} -> {sum}");
    }
}

#[cfg(test)]
mod tests;
