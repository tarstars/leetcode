#[path = "../../sol_02.rs"]
mod sol_02;

struct Solution;

fn main() {
    for n in [1, 3, 4] {
        let matrix = Solution::generate_matrix(n);
        println!("n = {n}: {matrix:?}");
    }
}

#[cfg(test)]
#[path = "../tests.rs"]
mod tests;
