#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    for (candidates, target) in [(vec![2, 3, 6, 7], 7), (vec![2, 3, 5], 8)] {
        let combinations = Solution::combination_sum(candidates.clone(), target);
        println!("{candidates:?}, target {target} -> {combinations:?}");
    }
}

#[cfg(test)]
mod tests;
