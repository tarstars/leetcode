#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    for n in [0, 1, 2, 3, 4, 10, 30] {
        println!("F({n}) = {}", Solution::fib(n));
    }
}

#[cfg(test)]
mod tests;
