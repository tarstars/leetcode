#[path = "../../sol_01.rs"]
mod sol_01;

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
