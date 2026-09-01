#[path = "../../sol_01.rs"]
mod sol_01;

struct Solution;

fn main() {
    for n in [1, 4] {
        let solutions = Solution::solve_n_queens(n);
        println!("n = {n}: {solutions:?}");
    }
}

#[cfg(test)]
#[path = "../tests.rs"]
mod tests;
