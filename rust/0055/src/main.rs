#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    for nums in [vec![2, 3, 1, 1, 4], vec![3, 2, 1, 0, 4]] {
        let reachable = Solution::can_jump(nums.clone());
        println!("{nums:?} -> {reachable}");
    }
}

#[cfg(test)]
mod tests;
