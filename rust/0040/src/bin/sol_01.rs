#[path = "../../sol_01.rs"]
mod sol_01;

struct Solution;

fn main() {
    for (candidates, target) in [(vec![10, 1, 2, 7, 6, 1, 5], 8), (vec![2, 5, 2, 1, 2], 5)] {
        let combinations = Solution::combination_sum2(candidates.clone(), target);
        println!("{candidates:?}, target {target} -> {combinations:?}");
    }
}

#[cfg(test)]
#[path = "../tests.rs"]
mod tests;
