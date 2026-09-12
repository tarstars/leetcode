#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    for (n, k) in [(4, 2), (1, 1), (5, 1), (4, 4), (5, 3)] {
        let combinations = Solution::combine(n, k);
        println!(
            "n = {n}, k = {k}: {} -> {combinations:?}",
            combinations.len()
        );
    }
}

#[cfg(test)]
mod tests;
