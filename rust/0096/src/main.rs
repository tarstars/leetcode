#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    for n in 1..=19 {
        println!("n = {n:2} -> {}", Solution::num_trees(n));
    }
}

#[cfg(test)]
mod tests;
