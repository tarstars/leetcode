#[path = "../../sol_01.rs"]
mod sol_01;

struct Solution;

fn main() {
    for (nums, target) in [
        (vec![2, 5, 6, 0, 0, 1, 2], 0),
        (vec![2, 5, 6, 0, 0, 1, 2], 3),
    ] {
        println!(
            "{nums:?}, target = {target}: {}",
            Solution::search(nums.clone(), target)
        );
    }
}

#[cfg(test)]
#[path = "../tests.rs"]
mod tests;
