#[path = "../../sol_01.rs"]
mod sol_01;

struct Solution;

fn main() {
    for mut nums in [
        vec![2, 0, 2, 1, 1, 0],
        vec![2, 0, 1],
        vec![1],
        vec![2, 2, 2, 0, 0, 0],
    ] {
        let before = nums.clone();
        Solution::sort_colors(&mut nums);
        println!("{before:?} -> {nums:?}");
    }
}

#[cfg(test)]
#[path = "../tests.rs"]
mod tests;
