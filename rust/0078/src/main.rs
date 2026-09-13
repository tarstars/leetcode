#[path = "../sol_01.rs"]
mod sol_01;

struct Solution;

fn main() {
    for nums in [vec![1, 2, 3], vec![0], vec![-1, 5], vec![3, 1, 2]] {
        let subsets = Solution::subsets(nums.clone());
        println!("{nums:?} -> {} subsets: {subsets:?}", subsets.len());
    }
}

#[cfg(test)]
mod tests;
