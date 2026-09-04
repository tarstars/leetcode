#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    for (m, n) in [(3, 7), (3, 2), (1, 1), (18, 17)] {
        println!("m = {m}, n = {n}: {}", Solution::unique_paths(m, n));
    }
}

#[cfg(test)]
mod tests;
