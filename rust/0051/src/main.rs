#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    for n in [1, 4] {
        let solutions = Solution::solve_n_queens(n);
        println!("n = {n}: {solutions:?}");
    }
}

#[cfg(test)]
mod tests;
