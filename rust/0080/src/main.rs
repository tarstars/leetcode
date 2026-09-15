#[path = "../sol_01.rs"]
mod sol_01;

struct Solution;

fn main() {
    for mut nums in [vec![1, 1, 1, 2, 2, 3], vec![0, 0, 1, 1, 1, 1, 2, 3, 3]] {
        let k = Solution::remove_duplicates(&mut nums) as usize;
        println!("k = {k}, prefix = {:?}", &nums[..k]);
    }
}

#[cfg(test)]
mod tests;
