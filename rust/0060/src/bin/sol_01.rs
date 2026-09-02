#[path = "../../sol_01.rs"]
mod sol_01;

struct Solution;

fn main() {
    for (n, k) in [(3, 3), (4, 9), (3, 1)] {
        let permutation = Solution::get_permutation(n, k);
        println!("n = {n}, k = {k}: {permutation}");
    }
}

#[cfg(test)]
#[path = "../tests.rs"]
mod tests;
