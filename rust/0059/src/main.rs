#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    for n in [1, 3, 4] {
        let matrix = Solution::generate_matrix(n);
        println!("n = {n}: {matrix:?}");
    }
}

#[cfg(test)]
mod tests;
