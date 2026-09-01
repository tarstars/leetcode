#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    for n in [1, 4] {
        let count = Solution::total_n_queens(n);
        println!("n = {n}: {count} solutions");
    }
}

#[cfg(test)]
mod tests;
