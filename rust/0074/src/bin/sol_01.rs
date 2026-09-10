#[path = "../../sol_01.rs"]
mod sol_01;

struct Solution;

fn main() {
    let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];

    for target in [3, 13, 1, 60, 0, 61] {
        let found = Solution::search_matrix(matrix.clone(), target);
        println!("target {target:3} -> {found}");
    }
}

#[cfg(test)]
#[path = "../tests.rs"]
mod tests;
