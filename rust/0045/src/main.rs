#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    for nums in [vec![2, 3, 1, 1, 4], vec![2, 3, 0, 1, 4]] {
        let jumps = Solution::jump(nums.clone());
        println!("{nums:?} -> {jumps}");
    }
}

#[cfg(test)]
mod tests;
